/// Settings-related errors.
#[derive(Debug, thiserror::Error)]
pub(crate) enum SettingsError {
    #[error("Invalid Environment: {0}")]
    InvalidEnvironment(String),

    #[error("Configuration Error: {0}")]
    ConfigError(String),

    #[error("EnvVar Error: {0}")]
    VarError(#[from] ::std::env::VarError),
}

pub(crate) type SettingsResult<T> = Result<T, SettingsError>;
