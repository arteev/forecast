use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid config file: {}", _0)]
    InvalidConfig(#[from] toml::de::Error),

    #[error("Invalid config file: {0}")]
    InvalidConfigCheck (String),

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

    #[error("Failed to render: {}", _0)]
    FailedRender(#[from] handlebars::RenderError),

    #[error("failed to parse: {}", _0)]
    Parse(#[from] serde_json::Error),

    #[error("Cache error: {0}")]
    InvalidCache(String),

}
