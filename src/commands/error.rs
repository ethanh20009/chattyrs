use super::{ask, weigh_in};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ask command failed, {0}")]
    AskError(#[from] ask::Error),
    #[error("Weigh in command failed, {0}")]
    WeighInError(#[from] weigh_in::Error),
    #[error("Command not implemented")]
    CommandNotImplemented,
}
