use std::sync::LazyLock;

use chrono::Utc;
use reqwest::{Body, Client, Response};
use serde::Serialize;
use serde_json::json;
use settings::prelude::CONFIG;
use tokio::sync::mpsc;

#[derive(Debug, Serialize)]
pub struct Log {
    level: String,
    message: String,
}

impl Log {
    #[must_use]
    pub fn new(level: &str, message: &str) -> Self {
        Self {
            level: level.to_owned(),
            message: message.to_owned(),
        }
    }
}

pub async fn send(level: &str, message: &str) {
    static LOG_CHANNEL: LazyLock<mpsc::Sender<Log>> = LazyLock::new(|| {
        let (tx, rx) = mpsc::channel(100);
        tokio::spawn(async move {
            act(rx).await;
        });
        tx
    });

    LOG_CHANNEL
        .send(Log::new(level, message))
        .await
        .unwrap_or_else(|e| eprintln!("Failed to send log: {e}",));
}

async fn act(mut rx: mpsc::Receiver<Log>) {
    let url = CONFIG.elasticsearch.url();
    let client = Client::new();
    while let Some(log) = rx.recv().await {
        let payload = json!({
            "level": log.level,
            "message": log.message,
            "timestamp": Utc::now().to_rfc3339(),
        });

        let body = Body::from(payload.to_string());

        let result = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(body)
            .send()
            .await
            .and_then(Response::error_for_status);

        if let Err(e) = result {
            eprintln!("Failed to send log: {e}");
        }
    }
}
