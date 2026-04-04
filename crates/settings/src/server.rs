use core::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use serde::{Deserialize, Serialize};

/// The server configuration.
///
/// This struct keeps all settings specific to the server – the interfaces
/// the server binds to, the port, an optional base URL, and the request
/// timeout. The struct is provided pre-defined by this project and cannot be
/// changed. It **must** be used for the `server` field in the
/// server-specific [`Settings`] struct:
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
pub struct ServerSettings {
    /// The IPv4 address to bind to, e.g. `127.0.0.1` or `0.0.0.0`
    pub host_v4: Ipv4Addr,
    /// The IPv6 address to bind to, e.g. `::1` or `::`
    pub host_v6: Ipv6Addr,
    /// The port to bind to, e.g. `8080`
    pub port: u16,
    /// The base URL of the server, e.g. `"https://api.myapp.com"`.
    ///
    /// When omitted, defaults to `"http://localhost:<port>"`. Useful to
    /// override when running behind a reverse proxy or with a custom domain.
    #[serde(default)]
    pub base_url: Option<String>,
    /// The timeout for requests in seconds
    pub timeout: u64,
}

impl Default for ServerSettings {
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

impl ServerSettings {
    /// Returns the IPv4 socket address the server binds to.
    ///
    /// This can be used when creating a TCP listener:
    ///
    /// ```rust
    /// let config: Settings = load_config(Env::Development);
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
    /// let config: Settings = load_config(Env::Development);
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
    /// let config: Settings = load_config(Env::Development);
    /// let [v4, v6] = config.server.addrs();
    ///
    /// let (l4, l6) = tokio::try_join!(TcpListener::bind(v4), TcpListener::bind(v6),)?;
    /// ```
    #[must_use]
    pub const fn addrs(&self) -> [SocketAddr; 2] { [self.addr_v4(), self.addr_v6()] }

    /// Returns the base URL of the server.
    ///
    /// If [`base_url`](ServerSettings::base_url) is set in the configuration
    /// it is returned as-is. Otherwise a URL is derived from the port:
    ///
    /// ```rust
    /// let mut config = ServerSettings::default(); // port = 8080
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
