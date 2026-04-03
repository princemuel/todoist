use core::fmt;
use core::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use core::str::FromStr;
use core::time::Duration;
use std::env;
use std::path::PathBuf;

use dotenvy::dotenv;
use figment::Figment;
use figment::providers::{Env as FigmentEnv, Format, Serialized, Toml};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use shared::errors::{Error, ErrorStatus};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

/// The application configuration.
///
/// This struct is the central point for the entire application configuration.
/// It holds the [`ServerConfig`] as well as [`DatabaseConfig`]and can be
/// extended with any application-specific configuration settings that will be
/// read from the main `Config.toml` and the environment-specific configuration
/// files.
///
/// For any setting that appears in both the `Config.toml` and the
/// environment-specific file, the latter will override the former so that
/// default settings can be kept in `Config.toml` that are overridden per
/// environment if necessary.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// whether to run the application in debug mode, which typically means more
    pub debug: bool,
    /// the server configuration: [`ServerConfig`]
    pub server: ServerConfig,
    /// the database configuration: [`DatabaseConfig`]
    pub database: DatabaseConfig,
    /// the redis configuration: [`RedisConfig`]
    pub redis: CacheConfig,
    /// the authentication configuration: [`AuthConfig`]
    pub auth: AuthConfig,
    // /// the OAuth configuration: [`OAuthConfig`]
    // pub oauth: OAuthConfig,
}

/// The server configuration.
///
/// This struct keeps all settings specific to the server – the interfaces the
/// server binds to, the port, an optional base URL, and the request timeout.
/// The struct is provided pre-defined by this project and cannot be changed. It
/// **must** be used for the `server` field in the application-specific
/// [`Config`] struct:
///
/// ```rust
/// #[derive(Clone, Debug, Deserialize)]
/// pub struct Config {
///     #[serde(default)]
///     pub server: ServerConfig,
///     pub database: DatabaseConfig,
///     // add your config settings here…
/// }
/// ```
///
/// Both IPv4 and IPv6 addresses are supported simultaneously. If `base_url` is
/// omitted from the configuration file it is derived automatically from the
/// port:
///
/// ```toml
/// [server]
/// host_v4 = "0.0.0.0"
/// host_v6 = "::"
/// port    = 8080
/// timeout = 10
/// # base_url omitted — defaults to "http://localhost:<port>"
/// ```
///
/// To override the derived URL (e.g. when running behind a reverse proxy):
///
/// ```toml
/// [server]
/// port     = 8080
/// base_url = "https://api.myapp.com"
/// timeout  = 10
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    /// The IPv4 address to bind to, e.g. `127.0.0.1` or `0.0.0.0`
    pub host_v4: Ipv4Addr,
    /// The IPv6 address to bind to, e.g. `::1` or `::`
    pub host_v6: Ipv6Addr,
    /// The port to bind to, e.g. `8080`
    pub port: u16,
    /// The base URL of the application, e.g. `"https://api.myapp.com"`.
    ///
    /// When omitted, defaults to `"http://localhost:<port>"`. Useful to
    /// override when running behind a reverse proxy or with a custom domain.
    #[serde(default)]
    pub base_url: Option<String>,
    /// The timeout for requests in seconds
    pub timeout: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host_v4: Ipv4Addr::UNSPECIFIED,
            host_v6: Ipv6Addr::UNSPECIFIED,
            port: 8080,
            base_url: None,
            timeout: 10,
        }
    }
}

impl ServerConfig {
    /// Returns the IPv4 socket address the server binds to.
    ///
    /// This can be used when creating a TCP listener:
    ///
    /// ```rust
    /// let config: Config = load_config(Env::Development);
    /// let listener = TcpListener::bind(config.server.addr_v4()).await?;
    /// ```
    #[must_use]
    pub const fn addr_v4(&self) -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(self.host_v4, self.port))
    }

    /// Returns the IPv6 socket address the server binds to.
    ///
    /// This can be used when creating a TCP listener:
    ///
    /// ```rust
    /// let config: Config = load_config(Env::Development);
    /// let listener = TcpListener::bind(config.server.addr_v6()).await?;
    /// ```
    #[must_use]
    pub const fn addr_v6(&self) -> SocketAddr {
        SocketAddr::V6(SocketAddrV6::new(self.host_v6, self.port, 0, 0))
    }

    /// Returns both socket addresses as an array, IPv4 first.
    ///
    /// Convenient when spawning two listeners concurrently:
    ///
    /// ```rust
    /// let config: Config = load_config(Env::Development);
    /// let [v4, v6] = config.server.addrs();
    ///
    /// let (l4, l6) = tokio::try_join!(TcpListener::bind(v4), TcpListener::bind(v6),)?;
    /// ```
    #[must_use]
    pub const fn addrs(&self) -> [SocketAddr; 2] { [self.addr_v4(), self.addr_v6()] }

    /// Returns the base URL of the application.
    ///
    /// If [`base_url`](ServerConfig::base_url) is set in the configuration it
    /// is returned as-is. Otherwise a URL is derived from the port:
    ///
    /// ```rust
    /// let mut config = ServerConfig::default(); // port = 8080
    /// assert_eq!(config.base_url(), "http://localhost:8080");
    ///
    /// config.base_url = Some("https://api.myapp.com".to_string());
    /// assert_eq!(config.base_url(), "https://api.myapp.com");
    /// ```
    #[must_use]
    pub fn base_url(&self) -> String {
        self.base_url
            .clone()
            .unwrap_or_else(|| format!("http://localhost:{}", self.port))
    }
}

/// The database configuration.
///
/// This struct keeps all settings specific to the database – currently that is
/// the database URL to use to connect to the database but more might be added
/// in the future. The struct is provided pre-defined by this project and cannot
/// be changed. It **must** be used for the `database` field in the
/// application-specific [`Config`] struct:
///
/// ```rust
/// #[derive(Deserialize, Clone, Debug)]
/// pub struct Config {
///     #[serde(default)]
///     pub server: ServerConfig,
///     pub database: DatabaseConfig,
///     // add your config settings here…
/// }
/// ```
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
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

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            name: "pennies".to_string(),
            username: "kalel".to_string(),
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

impl DatabaseConfig {
    /// Returns the URL to use to connect to the database, e.g.
    /// "<postgresql://user:password@localhost:5432/database>"
    /// This is constructed from the individual fields of the struct and can be
    /// used to connect to the database:
    /// ```rust
    /// let config: Config = load_config(Env::Development);
    /// let pool = PgPoolOptions::new()
    ///     .connect(config.database.url().as_str())
    ///     .await?;
    /// ```
    #[must_use]
    pub fn url(&self) -> String {
        let Self {
            host,
            port,
            name,
            username,
            password,
            ..
        } = self;

        let password = password.expose_secret();
        let ssl_mode = if self.require_ssl {
            "require"
        } else {
            "prefer"
        };
        format!("postgresql://{username}:{password}@{host}:{port}/{name}?sslmode={ssl_mode}",)
    }

    /// Returns the pool options to use when creating a connection pool with
    /// `sqlx::postgres::PgPoolOptions`. This includes settings like the minimum
    /// and maximum number of connections in the pool, the acquire timeout, etc.
    /// This can be used like this:
    /// ```rust
    /// let config: Config = load_config(Env::Development);
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
    /// let config: Config = load_config(Env::Development);
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
    /// let config: Config = load_config(Env::Development);
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

#[derive(Clone, Debug, Deserialize)]
pub struct CacheConfig {
    pub host: String,
    pub port: u16,
}

impl CacheConfig {
    #[must_use]
    pub fn uri(&self) -> String { format!("redis://{}:{}", self.host, self.port) }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: SecretString,  // APP_AUTH__JWT_SECRET
    pub hmac_secret: SecretString, // APP_AUTH__HMAC_SECRET
    pub jwt_expiry_secs: u64,
}

// #[derive(Clone, Debug, Deserialize)]
// pub struct OAuthConfig {
//     pub google: OAuthProviderConfig,
//     pub github: OAuthProviderConfig,
// }

// #[derive(Clone, Debug, Deserialize)]
// pub struct OAuthProviderConfig {
//     pub client_id: String,     // APP_OAUTH__GOOGLE__CLIENT_ID
//     pub client_secret: String, // APP_OAUTH__GOOGLE__CLIENT_SECRET
//     pub redirect_path: String,
// }

// impl OAuthProviderConfig {
//     #[must_use]
//     pub fn redirect_uri(&self, base_url: &str) -> String {
//         format!("{}{}", base_url, self.redirect_path)
//     }
// }

/// Loads the application configuration for a particular environment.
///
/// Depending on the environment, this function will behave differently:
/// * for [`Env::Development`], the function will load env vars from a `.env`
///   file at the project root if that is present
/// * for [`Env::Test`], the function will load env vars from a `.env.test` file
///   at the project root if that is present
/// * for [`Env::Production`], the function will only use the process env vars,
///   and not load a `.env` file
///
/// In case the .env or .env.test files live in another directory,
/// you can set that location using the `APP_ENVDIR` environment
/// variable. This is useful when they are mounted at separate locations in a
/// Docker container, for example.
///
/// Configuration settings are loaded from these sources (in that order so that
/// latter sources override former):
/// * the `config/Config.toml` file
/// * the `config/environments/<development|production|test>.toml` files
///   depending on the environment
/// * environment variables
///
/// # Errors
///
/// Returns an error if:
/// * the `.env` or `.env.test` file cannot be read or parsed
/// * any of the configuration TOML files cannot be read or parsed
/// * environment variables cannot be parsed into the expected types
/// * deserialization into the type `T` fails
pub fn load_config<'a, T>(env: &Env) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let envdir = option_env!("CARGO_MANIFEST_DIR").map(PathBuf::from);

    match (env, envdir) {
        (Env::Development, None) => {
            dotenv().ok();
        }
        (Env::Test, None) => {
            dotenvy::from_filename(".env.test").ok();
        }
        (Env::Development, Some(envdir)) => {
            dotenvy::from_filename(envdir.join(".env")).ok();
        }
        (Env::Test, Some(envdir)) => {
            dotenvy::from_filename(envdir.join(".env.test")).ok();
        }
        _ => { /* don't use any .env file for production */ }
    }

    let file = format!("{}.toml", env.as_str());
    let config: T = Figment::new()
        .merge(Serialized::defaults(ServerConfig::default()).key("server"))
        .merge(Toml::file("config/Config.toml"))
        .merge(Toml::file(format!("config/config/{file}")))
        .merge(FigmentEnv::prefixed("APP_").split("__"))
        .extract()
        .map_err(|e| Error::new(e.to_string(), ErrorStatus::Unknown))?;

    Ok(config)
}

/// The environment the application runs in.
///
/// The application can run in 3 different environments: development,
/// production, and test. Depending on the environment, the configuration might
/// be different (e.g. different databases) or the application might behave
/// differently.
#[derive(Copy, Clone, Debug, Default)]
pub enum Env {
    /// The development environment is what developers would use locally.
    #[default]
    Development,
    /// The production environment would typically be used in the released,
    /// user-facing deployment of the app.
    Production,
    /// The test environment is using when running e.g. `cargo test`
    Test,
}

impl Env {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
            Self::Test => "test",
        }
    }
}

/// Parses an [`Env`] from a string.
///
/// The environment can be passed in different forms, e.g. "dev", "development",
/// "prod", etc.
/// # Errors
///
/// If an invalid environment is passed, an error is returned.
impl FromStr for Env {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "dev" | "development" | "local" => Ok(Self::Development),
            "prod" | "production" => Ok(Self::Production),
            "test" => Ok(Self::Test),
            other => Err(Error::new(
                format!("Unknown environment: '{other}'!"),
                ErrorStatus::Unknown,
            )),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}

/// Returns the currently active environment.
///
/// If the `APP_ENVIRONMENT` env var is set, the application environment is
/// parsed from that (which might fail if an invalid environment is set).
/// # Errors
///
/// If the `APP_ENVIRONMENT` env var is set but contains an invalid environment,
/// an error is returned. If the env var is not set, no error is returned and
/// the environment defaults to [`Env::Development`].
pub fn get_env() -> Result<Env, Error> {
    if let Ok(env) = env::var("APP_ENVIRONMENT") {
        env.parse()
    } else {
        Ok(Env::default())
    }
}
