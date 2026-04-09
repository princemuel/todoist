use core::net::Ipv4Addr;

use serde::Deserialize;

/// The Elasticsearch configuration.
///
/// This struct keeps settings for connecting to Elasticsearch, including
/// the host, port, and index path for logging.
#[derive(Clone, Debug, Deserialize)]
pub struct ElasticSearchSettings {
    /// The Elasticsearch host, e.g. "localhost" or
    /// "elasticsearch.example.com"
    pub host: Ipv4Addr,
    /// The Elasticsearch port, e.g. 9200
    pub port: u16,
    /// The index path for logs, e.g. "logs/_doc"
    pub index: String,
}

impl ElasticSearchSettings {
    /// Returns the full Elasticsearch URL, e.g.
    /// "<http://localhost:9200/logs>/_doc"
    ///
    /// # Example
    ///
    /// ```rust
    /// let es = ElasticSearchSettings {
    ///     host: "127.0.0.1".into(),
    ///     port: 9200,
    ///     index: "logs/_doc".to_owned(),
    /// };
    /// assert_eq!(es.url(), "http://127.0.0.1:9200/logs/_doc");
    /// ```
    #[must_use]
    pub fn url(&self) -> String { format!("http://{}:{}/{}", self.host, self.port, self.index) }
}
