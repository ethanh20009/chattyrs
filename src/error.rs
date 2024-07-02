use crate::llm;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to load configuration, {source}")]
    Config {
        #[from]
        source: config::ConfigError,
    },
    #[error("Llm engine failed, {0}")]
    Llm(#[from] llm::error::Error),
}
