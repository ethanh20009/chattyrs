pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to load configuration")]
    Config {
        #[from]
        source: config::ConfigError,
    },
}
