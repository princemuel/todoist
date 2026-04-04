//! Prelude for the settings crate.
//!
//! This module provides a convenient way to import the most commonly used types
//! and functions from the settings crate.
//!
//! # Usage
//!
//! Instead of:
//! ```ignore
//! use settings::{CONFIG, Settings, Env, SettingsError, SettingsResult};
//! ```
//!
//! You can use:
//! ```ignore
//! use settings::prelude::*;
//! ```
//!
//! # What's Included
//!
//! - **`CONFIG`**: Global configuration instance (primary entry point)
//! - **`Settings`**: Main settings struct containing all sub-configurations
//! - **`Env`**: Environment enum (Development, Production, Test)
//! - **`get_env()`**: Function to detect the current environment
//! - **`load_config()`**: Function to manually load configuration
//! - **`SettingsError`**: Error type for configuration operations
//! - **`SettingsResult<T>`**: Result type for configuration operations
//! - **Sub-config types**: `ServerSettings`, `DatabaseSettings`,
//!   `CacheSettings`, `ElasticSearchSettings`, `AuthSettings`

use std::sync::LazyLock;

pub use crate::auth::AuthSettings;
pub use crate::cache::CacheSettings;
pub use crate::database::DatabaseSettings;
pub use crate::elasticsearch::ElasticSearchSettings;
use crate::env::get_env;
use crate::loader::load_config;
pub use crate::server::ServerSettings;
pub use crate::settings::Settings;

/// Global configuration instance.
///
/// This is lazily initialized on first access. If configuration loading fails,
/// the application will print an error and exit.
///
/// # Example
///
/// ```rust,no_run
/// use settings::CONFIG;
///
/// let db_url = CONFIG.database.url();
/// let server_addr = CONFIG.server.addr_v4();
/// ```
pub static CONFIG: LazyLock<Settings> = LazyLock::new(|| {
    load_config(get_env().unwrap_or_default()).unwrap_or_else(|e| {
        eprintln!("Fatal: Failed to load config: {e}");
        eprintln!("Application cannot start without valid configuration.");
        std::process::exit(1);
    })
});
