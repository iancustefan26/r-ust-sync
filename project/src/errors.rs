use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArgErrors {
    #[error("Invalid location format: {0}")]
    InvalidLocation(String),
}
