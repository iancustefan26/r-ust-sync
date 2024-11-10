use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandErrors {
    #[error("Command not found")]
    NotFound,
    #[error("Command spell incorrectly")]
    IncorrectSpell,
    #[error("Bad arguments")]
    BadArgs,
    #[error("Unexpected database error")]
    Unexpected,
    #[error("Stop")]
    Stop,
}
