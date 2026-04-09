use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, header};
use shared::errors::{Error, ErrorStatus};
use shared::safe_eject;
use shared::token::AuthToken;
use task_core::actions::delete::delete as delete_core;
use task_core::actions::get::get_all as get_all_core;
use task_dal::tasks::transactions::delete::DeleteOne;
use task_dal::tasks::transactions::get::GetAll;

/// Deletes an item from the tasks list by name.
///
/// # Arguments
/// - `req` - The HTTP request.
///
/// # Returns
/// List of task items
pub async fn delete<T: DeleteOne + GetAll>(
    name: &str,
    _token: AuthToken,
) -> Result<Response<Full<Bytes>>, Error> {
    delete_core::<T>(name).await?;

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
