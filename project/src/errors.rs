use thiserror::Error;

// Errors for given arguments (maybe more in the future but for now only this is needed)
#[derive(Debug, Error)]
pub enum ArgErrors {
    #[error("Invalid location format: {0}")]
    InvalidLocation(String),
    #[error("Config file empty, use --help to give --set arg some location paths")]
    EmptyCfg,
}

#[derive(Debug, Error)]
pub enum FileErrors {
    #[error("Invalid file for listing: {0}")]
    InvalidFileForListing(String),
}
