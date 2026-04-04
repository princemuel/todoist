use serde::Deserialize;

use crate::auth::AuthSettings;
use crate::cache::CacheSettings;
use crate::database::DatabaseSettings;
use crate::elasticsearch::ElasticSearchSettings;
use crate::server::ServerSettings;

/// The complete server configuration.
///
/// This struct is the central point for the entire server configuration.
/// It aggregates all sub-configurations (server, database, cache, etc.) and
/// serves as the single source of truth for all runtime settings.
///
/// Configuration is loaded using a layered approach (later sources override
/// earlier):
/// 1. **Defaults**: Built-in defaults from struct implementations
/// 2. **Config.toml**: Global defaults (values used across all environments)
/// 3. **Environment-specific override file**:
///    - `config/local.toml` for development (when
///      `APP_ENVIRONMENT=local|dev|development`)
///    - `config/prod.toml` for production (when
///      `APP_ENVIRONMENT=prod|production`)
///    - `config/test.toml` for testing (when `APP_ENVIRONMENT=test`)
/// 4. **Environment variables**: Prefixed with `APP_` (e.g.,
///    `APP_DATABASE__HOST`)
///    - Use double underscore `__` to denote nested structure (e.g.,
///      `DATABASE__HOST` → `[database] host`)
///
/// This ensures that global defaults are in `Config.toml`, only
/// environment-specific differences are in environment files, and
/// secrets/runtime values come from env vars.
#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    /// Whether to run the server in debug mode
    pub debug: bool,
    /// The server configuration
    pub server: ServerSettings,
    /// The database configuration
    pub database: DatabaseSettings,
    /// The cache (Redis) configuration
    pub cache: CacheSettings,
    /// The Elasticsearch configuration
    pub elasticsearch: ElasticSearchSettings,
    /// The authentication configuration
    pub auth: AuthSettings,
    // /// the OAuth configuration
    // pub oauth: OAuthSettings,
}
