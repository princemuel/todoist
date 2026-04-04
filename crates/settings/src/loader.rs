use dotenvy::dotenv;
use figment::Figment;
use figment::providers::{Env as FigmentEnv, Format, Serialized, Toml};
use serde::Deserialize;

use crate::env::Env;
use crate::error::{SettingsError, SettingsResult};
use crate::server::ServerSettings;

/// Loads the server configuration for a particular environment.
///
/// Depending on the environment, this function will behave differently:
/// * for [`Env::Development`], the function will load env vars from a `.env`
///   file at the project root if that is present
/// * for [`Env::Development`], the function will load env vars from
///   `.env.local` file at the repository root if that is present
/// * for [`Env::Test`], the function will load env vars from a `.env.test` file
///   at the repository root if that is present
/// * for [`Env::Production`], the function will only use the process env vars,
///   and not load a `.env` file (use your deployment platform's env vars
///   instead)
///
/// Configuration settings are loaded from these sources (in that order so
/// that latter sources override earlier ones):
/// 1. Built-in [`ServerSettings`] defaults
/// 2. `Config.toml` (global defaults for all environments)
/// 3. Environment-specific file:
///    - `config/local.toml` for development
///    - `config/prod.toml` for production
///    - `config/test.toml` for testing
/// 4. Environment variables prefixed with `APP_` (e.g.,
///    `APP_DATABASE__PASSWORD`)
///
/// # Errors
///
/// Returns an error if:
/// * the `.env` or `.env.test` file cannot be read or parsed
/// * any of the configuration TOML files cannot be read or parsed
/// * environment variables cannot be parsed into the expected types
/// * deserialization into the type `T` fails
pub(crate) fn load_config<'a, T>(env: Env) -> SettingsResult<T>
where
    T: Deserialize<'a>,
{
    // Load .env files from repository root (where Cargo.toml is)
    match env {
        Env::Development => {
            // Try .env.local first (local development with actual values)
            if dotenvy::from_filename(".env.local").is_err() {
                // Fall back to dotenv() which looks for .env in cwd and parent dirs
                dotenv().ok();
            }
        }
        Env::Test => {
            // Load test environment variables
            dotenvy::from_filename(".env.test").ok();
        }
        Env::Production => {
            // Production: don't load any .env files
            // All config must come from process environment variables set by
            // the deployment platform (e.g., Kubernetes, Docker, AWS Lambda,
            // etc.)
        }
    }

    let config_file = env.config_file();

    // Load configuration files (files are in crates/settings/ relative paths)
    let config: T = Figment::new()
        .merge(Serialized::defaults(ServerSettings::default()).key("server"))
        .merge(Toml::file("Config.toml"))                    // Global defaults
        .merge(Toml::file(format!("config/{config_file}")))  // Environment-specific overrides
        .merge(FigmentEnv::prefixed("APP_").split("__"))     // Environment variable overrides
        .extract()
        .map_err(|e| SettingsError::ConfigError(e.to_string()))?;

    Ok(config)
}
