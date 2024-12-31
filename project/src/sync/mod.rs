use anyhow::Result;
use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;

use crate::errors::*;
use crate::utils::*;

#[derive(Eq, PartialEq, Hash)]
pub enum LocTypes {
    Ftp(String),    // ftp:user:password@URL/path
    Zip(String),    // zip:/path/to/archive.zip
    Folder(String), // folder:/path/to/folder
    SimpleFile(String),
}

pub trait ReadOnly {
    fn list_files(&self) -> Result<HashMap<LocTypes, (SystemTime, String)>>; // Returns file paths with last modified times
    fn read_file(&self) -> Option<Vec<u8>>; // Read files as bytes
}

pub trait ReadWrite: ReadOnly {
    fn write_file(&self, content: &[u8]) -> Result<()>;
    fn delete_file(&self) -> Result<()>;
}

impl ReadOnly for LocTypes {
    fn list_files(&self) -> Result<HashMap<LocTypes, (SystemTime, String)>> {
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

// Sync logic
pub struct Synchronizer {
    locations: Vec<Box<dyn ReadWrite>>,
}

impl Synchronizer {
    pub fn new(locations: Vec<Box<dyn ReadWrite>>) -> Self {
        Self { locations }
    }

    pub fn sync(&self) {
        self.initial_sync();

        loop {
            self.continous_sync();
        }
    }

    fn initial_sync(&self) {}

    fn continous_sync(&self) {}
}

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
