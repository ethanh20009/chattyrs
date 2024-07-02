use super::ask;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ask command failed, {0}")]
    AskError(#[from] ask::Error),
    #[error("Command not implemented")]
    CommandNotImplemented,
}
