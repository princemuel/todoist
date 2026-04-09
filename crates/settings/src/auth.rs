use secrecy::SecretString;
use serde::Deserialize;

/// The authentication configuration.
///
/// This struct keeps settings for JWT and HMAC authentication.
/// All secrets should be set via environment variables with the `APP_AUTH__`
/// prefix.
#[derive(Clone, Debug, Deserialize)]
pub struct AuthSettings {
    /// JWT signing secret (set via `APP_AUTH__JWT_SECRET` env var)
    pub jwt_secret: SecretString,
    /// HMAC signing secret (set via `APP_AUTH__HMAC_SECRET` env var)
    pub hmac_secret: SecretString,
    /// JWT token expiry time in seconds
    pub jwt_expiry_secs: u64,
}
