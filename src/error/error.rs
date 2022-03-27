use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid config file: {}", _0)]
    InvalidConfig(#[from] toml::de::Error),

    #[error("Missing config file")]
    MissingConfig,

    #[error("Failed read config file")]
    FailedReadConfig,
}