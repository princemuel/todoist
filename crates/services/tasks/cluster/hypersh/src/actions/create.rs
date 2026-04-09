use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response, header};
use shared::errors::{Error, ErrorStatus};
use shared::hyper_utils::extract_body::extract_body;
use shared::safe_eject;
use shared::token::AuthToken;
use task_core::actions::create::create as create_core;
use task_core::actions::get::get_all as get_all_core;
use task_dal::tasks::transactions::create::SaveOne;
use task_dal::tasks::transactions::get::GetAll;

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
pub async fn create<T: SaveOne + GetAll>(
    req: Request<Incoming>,
    _token: AuthToken,
) -> Result<Response<Full<Bytes>>, Error> {
    let payload = extract_body(req).await?;
    create_core::<T>(payload).await?;

    let body = safe_eject!(
        serde_json::to_string(&get_all_core::<T>().await?),
        ErrorStatus::Unknown
    )?;

    safe_eject!(
        Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body))),
        ErrorStatus::Unknown
    )
}
