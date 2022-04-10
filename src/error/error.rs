use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid config file: {}", _0)]
    InvalidConfig(#[from] toml::de::Error),

    #[error("Missing config file")]
    MissingConfig,

    #[error("Failed read config file")]
    FailedReadConfig,

    #[error("Failed to query: {}", _0)]
    HttpError(#[from] reqwest::Error),

    #[error("Invalid request: {text:?}({code:?})")]
    InvalidRequest {
        text: String,
        code: u16,
    },

    #[error("Invalid response")]
    InvalidResponse,
}