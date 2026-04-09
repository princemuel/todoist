use http_body_util::BodyExt as _;
use hyper::Request;
use hyper::body::{Buf as _, Incoming};
use serde::de::DeserializeOwned;

use crate::errors::{Error, ErrorStatus};
use crate::safe_eject;

/// Extracts the body from a request and
/// deserializes it into a struct.
///
/// # Arguments
/// * `req` - The request containing the JSON body.
///
/// # Returns
/// The deserialized struct.
///
/// # Errors
///
/// Will return a [`Error`] if it fails to parse
/// the json body
pub async fn extract_body<S: DeserializeOwned>(req: Request<Incoming>) -> Result<S, Error> {
    let buffer = safe_eject!(req.collect().await, ErrorStatus::BadRequest)?.aggregate();

    let body = safe_eject!(
        serde_json::from_reader(buffer.reader()),
        ErrorStatus::BadRequest,
        "Failed to parse JSON body"
    )?;

    Ok(body)
}
