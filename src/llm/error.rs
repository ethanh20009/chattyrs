pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create HTTP Client")]
    HTTPClientBuildFailed(#[from] reqwest::Error),
    #[error("Failed to get http response from ollama")]
    HTTPRequestFailed(String),
    #[error("Failed to parse http response from ollama")]
    HTTPResponseParseFailed,
}
