use http_body_util::BodyExt;
use hyper::Request;
use hyper::body::{Buf, Incoming};
use serde::de::DeserializeOwned;

use crate::errors::{SharedError, SharedErrorStatus};
use crate::safe_eject;

/// Extracts the body from a request and deserializes it into a struct.
///
/// # Arguments
/// * `req` - The request containing the JSON body.
///
/// # Returns
/// The deserialized struct.
///
/// # Errors
///
/// Will return a [`SharedError`] if it fails to parse the json body
pub async fn extract_body<S: DeserializeOwned>(
    req: Request<Incoming>,
) -> Result<S, SharedError> {
    let buffer = safe_eject!(req.collect().await, SharedErrorStatus::BadRequest)?.aggregate();

    let body = safe_eject!(
        serde_json::from_reader(buffer.reader()),
        SharedErrorStatus::BadRequest,
        "Failed to parse JSON body"
    )?;

    Ok(body)
}
