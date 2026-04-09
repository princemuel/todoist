#![cfg(feature = "sqlx-postgres")]

use core::time::Duration;

use secrecy::{ExposeSecret as _, SecretString};
use serde::Deserialize;
use sqlx::ConnectOptions as _;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

/// The database configuration.
///
/// This struct keeps all settings specific to the database, including
/// connection details, credentials, pool settings, and SSL requirements.
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseSettings {
    /// The host to connect to the database on, e.g. "localhost"
    pub host: String,
    /// The port to connect to the database on, e.g. 5432
    pub port: u16,
    /// The name of the database to connect to
    pub name: String,
    /// The username to use to connect to the database
    pub username: String,
    /// Whether to require SSL when connecting to the database
    pub require_ssl: bool,
    /// The password to use to connect to the database. This is a secret string
    pub password: SecretString,

    /// The minimum number of connections in the connection pool.
    pub pool_min_connections: u32,
    /// The maximum number of connections in the connection pool.
    pub pool_max_connections: u32,
    /// The maximum time to wait for acquiring a connection from the pool in
    /// milliseconds
    pub pool_acquire_timeout_ms: u64,
    /// The maximum time a connection can be idle in the pool before it is
    /// closed in milliseconds
    pub pool_idle_timeout_ms: u64,
    /// The maximum lifetime of a connection in the pool before it is closed in
    /// milliseconds
    pub pool_max_lifetime_ms: u64,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            host: "localhost".to_owned(),
            port: 5432,
            name: "pennies".to_owned(),
            username: "kalel".to_owned(),
            require_ssl: false,
            password: SecretString::from("p£AwJj6)e*]A13j0"),
            pool_min_connections: 1,
            pool_max_connections: 5,
            pool_acquire_timeout_ms: 5000,
            pool_idle_timeout_ms: 300_000,   // 5 minutes
            pool_max_lifetime_ms: 1_800_000, // 30 minutes
        }
    }
}

impl DatabaseSettings {
    /// Returns the URL to use to connect to the database, e.g.
    /// "<postgresql://user:password@localhost:5432/database>"
    /// This is constructed from the individual fields of the struct and can be
    /// used to connect to the database:
    /// ```rust
    /// let config: Settings = load_config(Env::Development);
    /// let pool = PgPoolOptions::new()
    ///     .connect(config.database.url().as_str())
    ///     .await?;
    /// ```
    #[must_use]
    pub fn url(&self) -> String { self.connect_opts_with_db().to_url_lossy().to_string() }

    /// Returns the pool options to use when creating a connection pool with
    /// `sqlx::postgres::PgPoolOptions`. This includes settings like the minimum
    /// and maximum number of connections in the pool, the acquire timeout, etc.
    /// This can be used like this:
    /// ```rust
    /// let config: Settings = load_config(Env::Development);
    /// let pool = config
    ///     .database
    ///     .pool_opts()
    ///     .connect_with(config.database.connect_opts())
    ///     .await?;
    /// ```
    #[must_use]
    pub fn pool_opts(&self) -> PgPoolOptions {
        PgPoolOptions::new()
            .min_connections(self.pool_min_connections)
            .max_connections(self.pool_max_connections)
            .acquire_timeout(Duration::from_millis(self.pool_acquire_timeout_ms))
            .idle_timeout(Duration::from_millis(self.pool_idle_timeout_ms))
            .max_lifetime(Duration::from_millis(self.pool_max_lifetime_ms))
    }

    /// Returns the connection options to connect to the database, including the
    /// database name. This can be used to create a connection pool:
    /// ```rust
    /// let config: Settings = load_config(Env::Development);
    /// let pool = PgPoolOptions::new()
    ///     .connect_with(config.database.connect_opts_with_db())
    ///     .await?;
    /// ```
    #[must_use]
    pub fn connect_opts_with_db(&self) -> PgConnectOptions {
        self.connect_opts().database(&self.name)
    }

    /// Returns the connection options to connect to the database, without the
    /// database name. This can be useful for admin operations like creating a
    /// database:
    /// ```rust
    /// let config: Settings = load_config(Env::Development);
    /// let conn = PgConnection::connect_with(config.database.connect_opts()).await?;
    ///
    /// sqlx::query("CREATE DATABASE my_database")
    ///     .execute(&conn)
    ///     .await?;
    /// ```
    #[must_use]
    pub fn connect_opts(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .ssl_mode(if self.require_ssl {
                PgSslMode::Require
            } else {
                PgSslMode::Prefer
            })
    }
}
