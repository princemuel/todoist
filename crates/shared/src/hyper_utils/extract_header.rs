use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};

use crate::errors::{Error, ErrorStatus};
use crate::token::AuthToken;

/// Extracts the token from the header of the
/// request.
///
/// # Arguments
/// * `req` - The request containing the token header
///
/// # Returns
/// the token from the header
///
/// # Errors
///
/// Will return a [`Response`] if it fails to
/// find or parse the token
#[expect(clippy::result_large_err)]
#[expect(clippy::unused_async)]
pub async fn extract_token(
    req: &Request<Incoming>,
) -> Result<AuthToken, Response<Full<Bytes>>> {
    let headers = req.headers();

    let token = match headers.get("token") {
        Some(token) => token
            .to_str()
            .map_err(|e| Error::new(e.to_string(), ErrorStatus::Unauthorized).into_response()),
        None => Err(Error::new(
            "token not found in request header".to_owned(),
            ErrorStatus::Unauthorized,
        )
        .into_response()),
    };

    Ok(AuthToken::new(token?.to_owned()))
}

#[macro_export]
macro_rules! extract_hyper_header_token {
    ($req:expr) => {
        match extract_token($req).await {
            Ok(token) => token,
            Err(error) => return Ok(error),
        }
    };
}
