use engine::actions::delete::delete as delete_core;
use engine::actions::get::get_all as get_all_core;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, header};
use shared::errors::{Error, ErrorStatus};
use shared::safe_eject;

/// Deletes an item from the tasks list by name.
///
/// # Arguments
/// - `req` - The HTTP request.
///
/// # Returns
/// List of task items
pub async fn delete(name: &str) -> Result<Response<Full<Bytes>>, Error> {
    delete_core(name)?;

    let body = safe_eject!(
        serde_json::to_string(&get_all_core()?),
        ErrorStatus::Unknown
    )?;

    safe_eject!(
        Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body))),
        ErrorStatus::Unknown
    )
}
