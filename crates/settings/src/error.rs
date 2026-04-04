/// Settings-related errors.
#[derive(Debug, thiserror::Error)]
pub(crate) enum SettingsError {
    #[error("Invalid environment: {0}")]
    InvalidEnvironment(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub(crate) type SettingsResult<T> = Result<T, SettingsError>;
