use core::net::Ipv4Addr;

use serde::Deserialize;

/// The cache (Redis) configuration.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct CacheSettings {
    /// The Redis host, e.g. "localhost" or "redis.example.com"
    pub host: Ipv4Addr,
    /// The Redis port, e.g. 6379
    pub port: u16,
}

impl CacheSettings {
    /// Returns the Redis URI for connecting to the cache.
    ///
    /// # Example
    ///
    /// ```rust
    /// let cache = CacheSettings {
    ///     host: "127.0.0.1".into(),
    ///     port: 6379,
    /// };
    /// assert_eq!(cache.uri(), "redis://127.0.0.1:6379");
    /// ```
    #[must_use]
    pub fn uri(&self) -> String { format!("redis://{}:{}", self.host, self.port) }
}
