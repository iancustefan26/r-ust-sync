use anyhow::Result;
use filetime::FileTime;
use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;

use crate::errors::*;
use crate::utils::*;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum LocTypes {
    Ftp(String),    // ftp:user:password@URL/path
    Zip(String),    // zip:/path/to/archive.zip
    Folder(String), // folder:/path/to/folder
    SimpleFile(String),
}

pub trait ReadOnly {
    fn list_files(&self) -> Result<HashMap<String, (LocTypes, SystemTime, String)>>; // Returns file paths with last modified times
    fn read_file(&self) -> Option<Vec<u8>>; // Read files as bytes
}

pub trait ReadWrite: ReadOnly {
    fn write_file(&self, content: &[u8]) -> Result<()>;
    fn delete_file(&self) -> Result<()>;
}

impl ReadOnly for LocTypes {
    fn list_files(&self) -> Result<HashMap<String, (LocTypes, SystemTime, String)>> {
        match self {
            LocTypes::Ftp(url) => {
                // FTP logic
                Ok(HashMap::new())
            }
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
            LocTypes::Ftp(url) => {
                // FTP logic
                None
            }
            LocTypes::Zip(path) => file_as_bytes(path),
            LocTypes::Folder(_) => None,
            LocTypes::SimpleFile(path) => file_as_bytes(path),
        }
    }
}

impl ReadWrite for LocTypes {
    fn write_file(&self, content: &[u8]) -> Result<()> {
        match self {
            LocTypes::Ftp(url) => Ok(()),
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
            LocTypes::Ftp(url) => Ok(()),
            LocTypes::Folder(path) => Ok(delete(path)?),
            LocTypes::Zip(_) => {
                Err(FileErrors::InvalidFileForDelete("ZIP file is read-only".to_string()).into())
            }
            LocTypes::SimpleFile(path) => Ok(delete(path)?),
        }
    }
}

fn duplicate_newer_file(
    file_1: (String, (LocTypes, SystemTime, String)),
    file_2: (String, (LocTypes, SystemTime, String)),
) -> Result<()> {
    let (newer_file, older_file) = {
        if file_1.1 .1 > file_2.1 .1 {
            println!("Found a match: {} {}", file_1.1 .0, file_2.1 .0);
            (file_1, file_2)
        } else if file_1.1 .1 < file_2.1 .1 {
            println!("Found a match: {} {}", file_1.1 .0, file_2.1 .0);
            (file_2, file_1)
        } else {
            return Ok(()); // Same file
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
        self.initial_sync()?;

        loop {
            self.continous_sync()?;
        }
    }

    fn initial_sync(&self) -> Result<()> {
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
                            (LocTypes::Ftp(_), LocTypes::Ftp(_)) => {}
                            (LocTypes::Ftp(_), LocTypes::Zip(_)) => {}
                            (LocTypes::Ftp(_), LocTypes::SimpleFile(_)) => {}
                            (LocTypes::Ftp(_), LocTypes::Folder(_)) => {}
                            (LocTypes::Zip(path), LocTypes::Folder(_)) => {
                                println!("ZIP-Folder");
                            }
                            (LocTypes::Zip(path), LocTypes::Ftp(_)) => {
                                println!("ZIP-FTP");
                            }
                            (LocTypes::Zip(path), LocTypes::SimpleFile(_)) => {
                                println!("ZIP: {path} - FILE");
                                if file_1.1 .1 > file_2.1 .1 {
                                    duplicate_newer_file_from_zip(
                                        file_1,
                                        (file_2.0.clone(), file_2.1.clone()),
                                    )?;
                                }
                            }
                            (LocTypes::Zip(path), LocTypes::Zip(_)) => {
                                println!("ZIP-ZIP");
                            }
                            (LocTypes::SimpleFile(_), LocTypes::Folder(_)) => {
                                println!("FILE-FOLDER");
                            }
                            (LocTypes::SimpleFile(_), LocTypes::SimpleFile(_)) => {
                                println!("FILE-FILE");
                                duplicate_newer_file(file_1, (file_2.0.clone(), file_2.1.clone()))?;
                            }
                            (LocTypes::SimpleFile(_), LocTypes::Ftp(_)) => {
                                println!("FILE-FTP");
                            }
                            (LocTypes::SimpleFile(_), LocTypes::Zip(_)) => {
                                println!("FILE-ZIP SKIP");
                            }
                            (LocTypes::Folder(path), LocTypes::Folder(_)) => {
                                println!("FOLDER-FOLDER SKIP");
                            }
                            (LocTypes::Folder(path), LocTypes::Zip(_)) => {
                                println!("FOLDER-FOLDER");
                            }
                            (LocTypes::Folder(path), LocTypes::Ftp(_)) => {
                                println!("FOLDER-FOLDER");
                            }
                            (LocTypes::Folder(path), LocTypes::SimpleFile(_)) => {
                                println!("FOLDER-FOLDER");
                            }
                        }
                    } else {
                        // If only a location contains a file, the file is duplicated to the other location
                    }
                }
            }
        }
        Ok(())
    }

    fn continous_sync(&self) -> Result<()> {
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
            LocTypes::Ftp(details) => write!(f, "{}", details),
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
