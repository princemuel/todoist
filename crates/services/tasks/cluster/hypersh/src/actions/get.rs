use task_core::actions::get::{get_all as get_all_core, get_by_name as get_by_name_core};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, header};
use shared::errors::{Error, ErrorStatus};
use shared::safe_eject;

/// Gets all tasks.
///
/// # Returns
/// A `Result` containing the response to the
/// request or an error
pub async fn get_all() -> Result<Response<Full<Bytes>>, Error> {
    let items = get_all_core()?;
    let body = safe_eject!(serde_json::to_string(&items), ErrorStatus::Unknown)?;
    safe_eject!(
        Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body))),
        ErrorStatus::Unknown
    )
}

/// Gets a task by name.
///
/// # Arguments
/// * `req`: The request the extract the name from the URL
///
/// # Returns
/// An `HttpResponse` with a JSON body containing of the task specified in the
/// URL
pub async fn get_by_name(name: &str) -> Result<Response<Full<Bytes>>, Error> {
    let item = get_by_name_core(name)?;
    let body = safe_eject!(serde_json::to_string(&item), ErrorStatus::Unknown)?;
    safe_eject!(
        Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body))),
        ErrorStatus::Unknown
    )
}
