use anyhow::Result;
use filetime::FileTime;
use ftp::{connect_to_ftp, put_file, read_ftp_file};
use notify::event::ModifyKind;
use notify::{RecursiveMode, Watcher};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc;
use std::time::SystemTime;
use std::{fmt, thread};

mod ftp;
pub mod modes;

use crate::utils::*;
use crate::{errors::*, utils};
use ftp::*;
use modes::{CreateType, SyncMode};

type FtpServers = Option<Vec<HashMap<String, (LocTypes, SystemTime, String)>>>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum LocTypes {
    Ftp(String, String, String, String), // ftp:user:password@URL/path
    Zip(String),                         // zip:/path/to/archive.zip
    Folder(String),                      // folder:/path/to/folder
    SimpleFile(String),                  // /path/to/folder/file.ext
}

// ReadOnly trait for the ZIP archives
pub trait ReadOnly {
    fn list_files(&self) -> Result<HashMap<String, (LocTypes, SystemTime, String)>>; // Returns file paths with last modified times
    fn read_file(&self) -> Option<Vec<u8>>; // Read files as bytes
}

// ReadWrite trait that extends ReadOnly for Folder and FTP locations because I can modify them
pub trait ReadWrite: ReadOnly {
    fn write_file(&self, content: &[u8]) -> Result<()>; // Write bytes into file
    fn delete_file(&self) -> Result<()>; // Delete the file
    fn create_file(&self, path: &str, create_type: CreateType) -> Result<()>; // Create a file in path of type create_type
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
            LocTypes::Ftp(_, _, _, _) => Ok(()),
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
            LocTypes::Ftp(user, pass, url, path) => Ok(delete_ftp_file(user, pass, url, path)?),
            LocTypes::Folder(path) => Ok(delete(path)?),
            LocTypes::Zip(_) => {
                Err(FileErrors::InvalidFileForDelete("ZIP file is read-only".to_string()).into())
            }
            LocTypes::SimpleFile(path) => Ok(delete(path)?),
        }
    }

    fn create_file(&self, path: &str, create_type: CreateType) -> Result<()> {
        let result_path = format!("{}/{}", self, path);
        match self {
            LocTypes::Ftp(user, pass, url, ftp_path) => {
                let result_path = format!("{}/{}", ftp_path, path);
                Ok(create_ftp_file(user, pass, url, &result_path, create_type)?)
            }
            LocTypes::Folder(_) => Ok(create(&result_path, create_type)?),
            LocTypes::Zip(_) => {
                Err(FileErrors::InvalidFileForWriting("ZIP file is read-only".to_string()).into())
            }
            LocTypes::SimpleFile(_) => Ok(create(&result_path, create_type)?),
        }
    }
}

// Function that duplicates the newer file to the locations that has the older file
// in case a file with the same name and path is found
fn duplicate_newer_file(
    file_1: (String, (LocTypes, SystemTime, String)),
    file_2: (String, (LocTypes, SystemTime, String)),
) -> Result<()> {
    let (newer_file, older_file) = {
        match file_1.1 .1.cmp(&file_2.1 .1) {
            Ordering::Greater => (file_1, file_2),
            Ordering::Less => (file_2, file_1),
            Ordering::Equal => return Ok(()), // Same file
        }
    };
    let bytes = newer_file.1 .0.read_file();
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

// Same thing as the above function but extracts the file from the zip only if the zip is newer
fn duplicate_newer_file_from_zip(
    newer_file: (String, (LocTypes, SystemTime, String)),
    older_file: (String, (LocTypes, SystemTime, String)),
) -> Result<()> {
    let bytes = newer_file.1 .0.read_file();
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

// Sync logic struct
pub struct Synchronizer {
    locations: Vec<LocTypes>,
    prev_ftp_files: FtpServers,
}

impl Synchronizer {
    // Retrieve new instance
    pub fn new(locations: Vec<LocTypes>, option: FtpServers) -> Self {
        Self {
            locations,
            prev_ftp_files: option,
        }
    }

    // main function
    pub fn sync(&mut self) -> Result<()> {
        self.initial_sync(SyncMode::Any)?;
        // Now all the locations should be synchronized
        loop {
            thread::spawn(|| match utils::perform_check() {
                Ok(_) => println!("Performed check!"),
                Err(e) => println!("Performing check failed: {}", e),
            });
            // Every X seconds if nothing happened perform a check (mostly for FTP stored files)
            // I will be doing this by creating a thread that creates a hidden file .temp_check and
            // deletes it very fast in order to trigger the file watcher so a complete sync check will be done
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
                        // This match has to do with edge-cases of different locations and syncing the newer file found
                        match (file_1.1 .0.clone(), file_2.1 .0.clone()) {
                            (LocTypes::SimpleFile(_), LocTypes::Ftp(user, pass, url, ftp_path)) => {
                                // PUT the file into FTP server
                                match file_1.1 .1.cmp(&file_2.1 .1) {
                                    std::cmp::Ordering::Greater => {
                                        // The SimpleFile is newer and I have to PUT it on the FTP server
                                        let file_bytes =
                                            file_1.1 .0.read_file().unwrap_or_default();
                                        put_file(&file_bytes, &user, &pass, &url, &ftp_path)?;
                                    }
                                    std::cmp::Ordering::Less => {
                                        // The FTP file is newer and needs to overwrite the local file
                                        let bytes = file_2.1 .0.read_file().unwrap_or_default();
                                        file_1.1 .0.write_file(&bytes)?;
                                        let last_modif_time =
                                            FileTime::from_system_time(file_2.1 .1);
                                        filetime::set_file_times(
                                            file_1.1 .0.to_string(),
                                            last_modif_time,
                                            last_modif_time,
                                        )?;
                                    }
                                    std::cmp::Ordering::Equal => {
                                        // same file
                                    }
                                }
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
                        // Also, the below matcher are dealing with different edge-cases where different locations are encountered
                        // in different sync manner(delete, modify, create)
                        match loc2 {
                            LocTypes::Ftp(user, pass, url, ftp_path) => match file_1.1 .0 {
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
                                                let new_ftp_file_path = format!(
                                                    "{}/{}",
                                                    ftp_path,
                                                    &file_1.0.to_string()
                                                );
                                                put_file(
                                                    &bytes,
                                                    user,
                                                    pass,
                                                    url,
                                                    &new_ftp_file_path,
                                                )?;
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

                                _ => {}
                            },
                            LocTypes::Folder(_) => match file_1.clone().1 .0 {
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
                                LocTypes::Ftp(_, _, _, _) => match mode {
                                    SyncMode::Delete => {
                                        file_1.clone().1 .0.delete_file()?;
                                    }
                                    _ => {
                                        let path = file_1.clone().0.to_string();
                                        let file_create =
                                            loc2.create_file(&path, CreateType::Folder);
                                        if file_create.is_err() {
                                            loc2.create_file(
                                                &file_1.0.to_string(),
                                                CreateType::File,
                                            )?;
                                            let bytes = file_1.clone().1 .0.read_file();
                                            match bytes {
                                                Some(bytes) => {
                                                    let new_file_path = format!(
                                                        "{}/{}",
                                                        loc2,
                                                        &file_1.0.to_string()
                                                    );
                                                    let new_file =
                                                        LocTypes::SimpleFile(new_file_path);
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
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // After initialization, this function performs a check to see if all the locations are synced
    // by creating a watcher for system files, and a X seconds GET for FTP servers
    // and sync the locations found by calling the above function in the corespondent mode
    fn continous_sync(&mut self) -> Result<()> {
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
            for loc in &self.locations {
                if let LocTypes::Ftp(_, _, url, _) = loc {
                    if let Some(ftp_servers) = &self.prev_ftp_files {
                        for sv in ftp_servers {
                            if let LocTypes::Ftp(_, _, url2, _) = &sv.iter().next().unwrap().1 .0 {
                                if url == url2 {
                                    let files = loc.list_files()?;
                                    if files.len() < sv.clone().len() {
                                        self.initial_sync(SyncMode::Delete)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self.prev_ftp_files = {
                let mut ftps = Vec::new();
                for loc in &self.locations {
                    if let LocTypes::Ftp(_, _, _, _) = loc {
                        ftps.push(loc.list_files()?);
                    }
                }
                if ftps.is_empty() {
                    None
                } else {
                    Some(ftps)
                }
            };
            match res {
                Ok(event) => {
                    for path in &event.paths {
                        if path.as_path().to_str().unwrap().contains(".DS") {
                            continue 'result_loop;
                        }
                    }
                    match event.kind {
                        notify::EventKind::Create(_) => {
                            self.initial_sync(SyncMode::Create)?;
                        }
                        notify::EventKind::Modify(modif_kind) => match modif_kind {
                            ModifyKind::Name(_) => {
                                self.initial_sync(SyncMode::Delete)?;
                            }
                            ModifyKind::Data(_) => {
                                println!("Modified: {:?}", event.paths);
                                self.initial_sync(SyncMode::Modify)?;
                            }
                            _ => {}
                        },
                        notify::EventKind::Remove(_) => {
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
