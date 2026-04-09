use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[cfg(feature = "elastic")]
pub use crate::logger::elastic::send;

/// Initializes the logger with a default subscriber that logs to stdout at INFO
/// level.
/// This should be called once at the start of the application, before any
/// logging calls are made.
///
/// # Panics
///
/// Panics if the default subscriber can't be set.
pub fn init() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    #[expect(clippy::expect_used)]
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logger");
}

pub async fn info(message: &str) {
    tracing::info!("{}", message);
    #[cfg(feature = "elastic")]
    send("INFO", message).await;
}
pub async fn warn(message: &str) {
    tracing::warn!("{}", message);
    #[cfg(feature = "elastic")]
    send("WARN", message).await;
}
pub async fn error(message: &str) {
    tracing::error!("{}", message);
    #[cfg(feature = "elastic")]
    send("ERROR", message).await;
}
pub async fn debug(message: &str) {
    tracing::debug!("{}", message);
    #[cfg(feature = "elastic")]
    send("DEBUG", message).await;
}
pub async fn trace(message: &str) {
    tracing::trace!("{}", message);
    #[cfg(feature = "elastic")]
    send("TRACE", message).await;
}
