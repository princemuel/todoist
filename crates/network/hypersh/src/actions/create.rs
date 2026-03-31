use engine::actions::create::create as create_one;
use engine::actions::get::get_all as get_all_core;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response, header};
use shared::errors::{Error, ErrorStatus};
use shared::hyper_utils::extract_body::extract_body;
use shared::safe_eject;

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
pub async fn create(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Error> {
    let payload = extract_body(req).await?;
    create_one(payload)?;

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
