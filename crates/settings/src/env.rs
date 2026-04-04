use core::fmt;
use core::str::FromStr;
use std::env;

use crate::error::SettingsError;

/// The environment the server runs in.
///
/// The application can run in 3 different environments: development,
/// production, and test. Each environment:
/// - Has its own config file (`config/local.toml`, `config/prod.toml`,
///   `config/test.toml`)
/// - May load different `.env` files (`.env.local`, `.env.test`)
/// - Uses different default values for database, cache, timeouts, etc.
/// - May have different security constraints (e.g., SSL requirements)
///
/// # Setting the Environment
///
/// Set the environment via the `APP_ENVIRONMENT` variable:
/// ```bash
/// # Development (local)
/// export APP_ENVIRONMENT=local  # or 'dev', 'development'
///
/// # Production
/// export APP_ENVIRONMENT=prod   # or 'production'
///
/// # Testing
/// export APP_ENVIRONMENT=test
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub(crate) enum Env {
    /// Local/development environment: relaxed constraints, localhost bindings,
    /// longer timeouts for debugging. Uses `config/local.toml` and
    /// `.env.local`. Set via: `APP_ENVIRONMENT=local` (or `dev`,
    /// `development`)
    #[default]
    Development,
    /// Production environment: strict security, SSL required, optimized for
    /// performance. Uses `config/prod.toml` and requires all secrets in process
    /// env vars. Set via: `APP_ENVIRONMENT=prod` (or `production`)
    Production,
    /// Test environment: for running `cargo test`. Uses `config/test.toml` and
    /// `.env.test`. Set via: `APP_ENVIRONMENT=test`
    Test,
}

impl Env {
    /// Returns a human-readable string representation of the environment.
    /// This is primarily for display/logging purposes.
    #[must_use]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
            Self::Test => "test",
        }
    }

    /// Returns the config file name for this environment.
    #[must_use]
    pub(crate) fn config_file(self) -> &'static str {
        match self {
            Self::Development => "local.toml",
            Self::Production => "prod.toml",
            Self::Test => "test.toml",
        }
    }
}

/// Parses an [`Env`] from a string.
///
/// The environment can be specified in multiple forms for convenience:
/// - Development: `"dev"`, `"development"`, `"local"`
/// - Production: `"prod"`, `"production"`
/// - Test: `"test"`
///
/// The parsing is case-insensitive and whitespace is trimmed.
///
/// # Errors
///
/// If an invalid environment string is passed, an error is returned with
/// details about what was received.
impl FromStr for Env {
    type Err = SettingsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "dev" | "development" | "local" => Ok(Self::Development),
            "prod" | "production" => Ok(Self::Production),
            "test" => Ok(Self::Test),
            other => Err(SettingsError::InvalidEnvironment(format!(
                "Unknown environment: '{other}'!"
            ))),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}

/// Returns the currently active environment.
///
/// If the `APP_ENVIRONMENT` env var is set, the server environment is
/// parsed from that (which might fail if an invalid environment is set).
///
/// # Errors
///
/// If the `APP_ENVIRONMENT` env var is set but contains an invalid environment,
/// an error is returned. If the env var is not set, no error is returned and
/// the environment defaults to [`Env::Development`].
pub(crate) fn get_env() -> Result<Env, SettingsError> {
    if let Ok(env) = env::var("APP_ENVIRONMENT") {
        env.parse()
    } else {
        Ok(Env::default())
    }
}
