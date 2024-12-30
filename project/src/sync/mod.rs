use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;

pub enum LocTypes {
    Ftp(String),    // ftp:user:password@URL/path
    Zip(String),    // zip:/path/to/archive.zip
    Folder(String), // folder:/path/to/folder
}

pub trait ReadOnly {
    fn list_files(&self) -> HashMap<String, SystemTime>; // Returns file paths with last modified times
    fn read_file(&self, path: &str) -> Option<Vec<u8>>; // Read files as bytes
}

pub trait ReadWrite: ReadOnly {
    fn write_file(&self, path: &str, content: &[u8]);
    fn delete_file(&self, path: &str);
}

impl ReadOnly for LocTypes {
    fn list_files(&self) -> HashMap<String, SystemTime> {
        match self {
            LocTypes::Ftp(url) => {
                // FTP logic
                HashMap::new()
            }
            LocTypes::Zip(path) => {
                // ZIP logic
                HashMap::new()
            }
            LocTypes::Folder(path) => {
                // Folder logic
                HashMap::new()
            }
        }
    }

    fn read_file(&self, path: &str) -> Option<Vec<u8>> {
        match self {
            LocTypes::Ftp(url) => {
                // FTP logic
                None
            }
            LocTypes::Zip(path) => {
                // ZIP logic
                None
            }
            LocTypes::Folder(path) => {
                // Folder logic
                None
            }
        }
    }
}

impl ReadWrite for LocTypes {
    fn write_file(&self, path: &str, content: &[u8]) {
        match self {
            LocTypes::Ftp(url) => {}
            LocTypes::Folder(path) => {}
            LocTypes::Zip(_) => {
                // Do nothing because ZIP is read only
                panic!("Cannot write to a ZIP archive.");
            }
        }
    }

    fn delete_file(&self, path: &str) {
        match self {
            LocTypes::Ftp(url) => {}
            LocTypes::Folder(path) => {}
            LocTypes::Zip(_) => {
                panic!("Cannot delete from a ZIP archive.");
            }
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
        }
    }
}
