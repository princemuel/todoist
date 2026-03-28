use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};

use crate::errors::{SharedError, SharedErrorStatus};
use crate::token::HeaderToken;

/// Extracts the token from the header of the request.
///
/// # Arguments
/// * `req` - The request containing the token header
///
/// # Returns
/// the token from the header
///
/// # Errors
///
/// Will return a [`Response`] if it fails to find or parse the token
#[allow(clippy::result_large_err)]
#[allow(clippy::unused_async)]
pub async fn extract_token(
    req: &Request<Incoming>,
) -> Result<HeaderToken, Response<Full<Bytes>>> {
    let headers = req.headers();

    let token = match headers.get("token") {
        Some(token) => token.to_str().map_err(|_| {
            SharedError::new(
                "token not a valid string".to_string(),
                SharedErrorStatus::Unauthorized,
            )
            .into_hyper_response()
        }),
        None => {
            return Err(SharedError::new(
                "token not found in request header".to_string(),
                SharedErrorStatus::Unauthorized,
            )
            .into_hyper_response());
        }
    };

    Ok(HeaderToken::new(token?.to_string()))
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
