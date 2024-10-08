pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create HTTP Client, {0}")]
    HTTPClientBuildFailed(#[from] reqwest::Error),
    #[error("Failed to get http response from ollama, {0}")]
    HTTPRequestFailed(String),
    #[error("Failed to parse http response from ollama, {0}")]
    HTTPResponseParseFailed(String),
    #[error("Empty response returned from LLM")]
    EmptyResponseError,
}
