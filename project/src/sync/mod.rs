use anyhow::Result;
use filetime::FileTime;
use ftp::{connect_to_ftp, read_ftp_file};
use notify::event::{CreateKind, ModifyKind, RenameMode};
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};

use crate::errors::*;
use crate::utils::*;

mod ftp;
pub mod modes;
pub use modes::{CreateType, SyncMode};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum LocTypes {
    Ftp(String, String, String, String), // ftp:user:password@URL/path
    Zip(String),                         // zip:/path/to/archive.zip
    Folder(String),                      // folder:/path/to/folder
    SimpleFile(String),
}

pub trait ReadOnly {
    fn list_files(&self) -> Result<HashMap<String, (LocTypes, SystemTime, String)>>; // Returns file paths with last modified times
    fn read_file(&self) -> Option<Vec<u8>>; // Read files as bytes
}

pub trait ReadWrite: ReadOnly {
    fn write_file(&self, content: &[u8]) -> Result<()>;
    fn delete_file(&self) -> Result<()>;
    fn create_file(&self, path: &str, create_type: CreateType) -> Result<()>;
}

impl ReadOnly for LocTypes {
    fn list_files(&self) -> Result<HashMap<String, (LocTypes, SystemTime, String)>> {
        match self {
            LocTypes::Ftp(user, pass, url, path) => Ok(connect_to_ftp(user, pass, url, path)?),
            LocTypes::Zip(path) => Ok(list_files_in_zip(path)?),
            LocTypes::Folder(path) => Ok(list_files_recursive(path)?),
            LocTypes::SimpleFile(_) => Err(FileErrors::InvalidFileForListing(
                "a simple file could not be iterated".to_string(),
            )
            .into()),
        }
    }

    fn read_file(&self) -> Option<Vec<u8>> {
        match self {
            LocTypes::Ftp(user, pass, url, path) => read_ftp_file(user, pass, url, path),
            LocTypes::Zip(path) => file_as_bytes(path),
            LocTypes::Folder(_) => None,
            LocTypes::SimpleFile(path) => file_as_bytes(path),
        }
    }
}

impl ReadWrite for LocTypes {
    fn write_file(&self, content: &[u8]) -> Result<()> {
        match self {
            LocTypes::Ftp(user, pass, url, path) => Ok(()),
            LocTypes::Folder(_) => Err(FileErrors::InvalidFileForWriting(
                "A folder can't be written".to_string(),
            )
            .into()),
            LocTypes::Zip(_) => {
                Err(FileErrors::InvalidFileForWriting("ZIP file is read-only".to_string()).into())
            }
            LocTypes::SimpleFile(path) => Ok(paste_to_file(path, content)?),
        }
    }

    fn delete_file(&self) -> Result<()> {
        match self {
            LocTypes::Ftp(user, pass, url, path) => Ok(()),
            LocTypes::Folder(path) => Ok(delete(path)?),
            LocTypes::Zip(_) => {
                Err(FileErrors::InvalidFileForDelete("ZIP file is read-only".to_string()).into())
            }
            LocTypes::SimpleFile(path) => Ok(delete(path)?),
        }
    }

    fn create_file(&self, path: &str, create_type: CreateType) -> Result<()> {
        let result_path = format!("{}/{}", self.to_string(), path);
        match self {
            LocTypes::Ftp(user, pass, url, path) => Ok(()),
            LocTypes::Folder(_) => Ok(create(&result_path, create_type)?),
            LocTypes::Zip(_) => {
                Err(FileErrors::InvalidFileForWriting("ZIP file is read-only".to_string()).into())
            }
            LocTypes::SimpleFile(_) => Ok(create(&result_path, create_type)?),
        }
    }
}

fn duplicate_newer_file(
    file_1: (String, (LocTypes, SystemTime, String)),
    file_2: (String, (LocTypes, SystemTime, String)),
) -> Result<()> {
    let (newer_file, older_file) = {
        match file_1.1 .1.cmp(&file_2.1 .1) {
            Ordering::Greater => {
                println!("Found a match: {} {}", file_1.1 .0, file_2.1 .0);
                (file_1, file_2)
            }
            Ordering::Less => {
                println!("Found a match: {} {}", file_1.1 .0, file_2.1 .0);
                (file_2, file_1)
            }
            Ordering::Equal => return Ok(()), // Same file
        }
    };
    let bytes = newer_file.1 .0.read_file();
    println!("Reading: {}", newer_file.1 .0.to_string());
    let last_modif_time = FileTime::from_system_time(newer_file.1 .1);
    match bytes {
        Some(bytes) => {
            older_file.1 .0.write_file(&bytes)?;
        }
        None => {
            return Err(FileErrors::InvalidFileForReading("Couldn't read file".to_string()).into());
        }
    };
    filetime::set_file_times(
        older_file.1 .0.to_string(),
        last_modif_time,
        last_modif_time,
    )?;

    Ok(())
}

fn duplicate_newer_file_from_zip(
    newer_file: (String, (LocTypes, SystemTime, String)),
    older_file: (String, (LocTypes, SystemTime, String)),
) -> Result<()> {
    let bytes = newer_file.1 .0.read_file();
    let last_modif_time = FileTime::from_system_time(newer_file.1 .1);
    match bytes {
        Some(bytes) => {
            older_file.1 .0.write_file(&bytes)?;
            println!("Found a match: {} {}", newer_file.1 .0, older_file.1 .0);
        }
        None => {
            return Err(FileErrors::InvalidFileForReading("Couldn't read file".to_string()).into());
        }
    };
    filetime::set_file_times(
        older_file.1 .0.to_string(),
        last_modif_time,
        last_modif_time,
    )?;

    Ok(())
}

// Sync logic
pub struct Synchronizer {
    locations: Vec<LocTypes>,
}

impl Synchronizer {
    pub fn new(locations: Vec<LocTypes>) -> Self {
        Self { locations }
    }

    pub fn sync(&self) -> Result<()> {
        for loc in &self.locations {
            let files = loc.list_files()?;
            for file in files {
                if let LocTypes::Ftp(_, _, _, _) = loc {
                    //let bytes = file.1 .0.read_file().unwrap();
                    println!(
                        "{} - {} - bytes : {:?}",
                        file.0,
                        file.1 .2,
                        file.1 .0.read_file().unwrap()
                    );
                }
            }
        }
        self.initial_sync(SyncMode::Any)?;
        // Now all the locations should be synchronized
        loop {
            // Listen for any changhes and sync
            println!("Loop");
            match self.continous_sync() {
                Ok(_) => println!("Quit."),
                Err(e) => println!("Encountered some error: {}", e),
            }
        }
    }

    fn initial_sync(&self, mode: modes::SyncMode) -> Result<()> {
        for loc1 in &self.locations {
            for loc2 in &self.locations {
                if loc2 == loc1 {
                    continue;
                }
                // Compare the 2 locations for intial sync
                let files1 = loc1.list_files()?;
                let files2 = loc2.list_files()?;
                for file_1 in files1 {
                    if files2.contains_key(&file_1.0) {
                        // O(1) (hashmap)
                        // If both locations contain a file, the newer version is copied
                        let file_2 = files2.get_key_value(&file_1.0).unwrap();
                        match (file_1.1 .0.clone(), file_2.1 .0.clone()) {
                            (
                                LocTypes::Ftp(user, pass, url, path),
                                LocTypes::Ftp(user2, pass2, url2, path2),
                            ) => {
                                // Extract the file from The FTP1 server and PUT it into FTP2 server
                            }
                            (LocTypes::SimpleFile(_), LocTypes::Ftp(user, pass, url, path)) => {
                                // PUT the file into FTP server
                            }
                            (LocTypes::Zip(_), LocTypes::Ftp(user, pass, url, path)) => {
                                // Extract the ZIP file and PUT it onto the FTP server if it's newer
                            }
                            (LocTypes::Ftp(user, pass, url, path), LocTypes::SimpleFile(_)) => {
                                // Extract the file from the FTP server if it's newer and replace it on my machine
                            }
                            (LocTypes::Zip(_), LocTypes::SimpleFile(_)) => {
                                if file_1.1 .1 > file_2.1 .1 {
                                    duplicate_newer_file_from_zip(
                                        file_1,
                                        (file_2.0.clone(), file_2.1.clone()),
                                    )?;
                                }
                            }
                            (LocTypes::SimpleFile(_), LocTypes::SimpleFile(_)) => {
                                duplicate_newer_file(file_1, (file_2.0.clone(), file_2.1.clone()))?;
                            }
                            _ => {}
                        }
                    } else {
                        // If only a location contains the file, the file is duplicated to the other location
                        match loc2 {
                            LocTypes::Ftp(user, pass, url, path) => {}
                            LocTypes::Folder(_) => match file_1.1 .0 {
                                LocTypes::Zip(_) | LocTypes::SimpleFile(_) => match mode {
                                    SyncMode::Delete => {
                                        // If only a location cotnains this file and SyncMode is set to delete
                                        // it means that from the other location someone deleted a file and has
                                        // to be deleted also from this location
                                        if let LocTypes::SimpleFile(_) = file_1.1 .0 {
                                            file_1.1 .0.delete_file()?;
                                        } else {
                                            self.initial_sync(SyncMode::Any)?;
                                        }
                                        // ZIP files are read-only so they can not be deleted
                                    }
                                    _ => {
                                        loc2.create_file(&file_1.0.to_string(), CreateType::File)?;
                                        let bytes = file_1.1 .0.read_file();
                                        match bytes {
                                            Some(bytes) => {
                                                let new_file_path =
                                                    format!("{}/{}", loc2, &file_1.0.to_string());
                                                let new_file = LocTypes::SimpleFile(new_file_path);
                                                new_file.write_file(&bytes)?;
                                            }
                                            None => {
                                                return Err(FileErrors::InvalidFileForReading(
                                                    "Couldn't read file".to_string(),
                                                )
                                                .into());
                                            }
                                        };
                                    }
                                },
                                LocTypes::Folder(_) => {
                                    if let SyncMode::Delete = mode {
                                        file_1.1 .0.delete_file()?;
                                    } else {
                                        loc2.create_file(
                                            &file_1.0.to_string(),
                                            CreateType::Folder,
                                        )?;
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn continous_sync(&self) -> Result<()> {
        let (tx, rx) = mpsc::channel::<Result<notify::Event, notify::Error>>(); // Correct type
        let mut watchers = Vec::new();
        for loc in &self.locations {
            if let LocTypes::Folder(path) = loc {
                let mut watcher = notify::recommended_watcher(tx.clone())?;
                watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
                watchers.push(watcher);
            }
        }

        'result_loop: for res in rx {
            match res {
                Ok(event) => {
                    for path in &event.paths {
                        if path.as_path().to_str().unwrap().contains(".DS") {
                            continue 'result_loop;
                        }
                    }
                    match event.kind {
                        notify::EventKind::Create(_) => {
                            println!("File created: {:?}", event.paths);
                            self.initial_sync(SyncMode::Create)?;
                        }
                        notify::EventKind::Modify(modif_kind) => match modif_kind {
                            ModifyKind::Name(_) => {
                                println!("File removed: {:?}", event.paths);
                                self.initial_sync(SyncMode::Delete)?;
                            }
                            ModifyKind::Data(_) => {
                                println!("File modified: {:?}", event.paths);
                                self.initial_sync(SyncMode::Modify)?;
                            }
                            _ => {}
                        },
                        notify::EventKind::Remove(_) => {
                            println!("File removed: {:?}", event.paths);
                            self.initial_sync(SyncMode::Delete)?;
                        }
                        _ => {}
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
        Ok(())
    }
}

/*
impl fmt::Display for LocTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocTypes::Ftp(_) => write!(f, "FTP Location:"),
            LocTypes::Zip(_) => write!(f, "ZIP Archive:"),
            LocTypes::Folder(_) => write!(f, "Folder Path:"),
            LocTypes::SimpleFile(_) => write!(f, "File Path:"),
        }
    }
}
    */

impl fmt::Display for LocTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocTypes::Ftp(user, pass, url, path) => write!(f, "{}:{}@{}/{}", user, pass, url, path),
            LocTypes::Zip(path) => write!(f, "{}", path),
            LocTypes::Folder(path) => write!(f, "{}", path),
            LocTypes::SimpleFile(path) => write!(f, "{}", path),
        }
    }
}

/*
impl fmt::Display for LocTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocTypes::Ftp(details) => write!(f, "FTP Location: {}", details),
            LocTypes::Zip(path) => write!(f, "ZIP Archive: {}", path),
            LocTypes::Folder(path) => write!(f, "Folder Path: {}", path),
            LocTypes::SimpleFile(path) => write!(f, "File Path: {}", path),
        }
    }
}
*/
