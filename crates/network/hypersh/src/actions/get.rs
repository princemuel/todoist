use engine::actions::get::get_all as get_all_tasks;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, header};

/// Gets all tasks.
pub async fn get_all() -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let items = get_all_tasks()?;
    let body = safe_eject!(
        serde_json::to_string(&items),
        NanoServiceErrorStatus::Unknown
    )?;

    safe_eject!(
        Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body))),
        NanoServiceErrorStatus::Unknown
    )
}
