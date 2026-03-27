use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use engine::actions::get::get_all as get_all_tasks;
use glue::errors::NanoServiceError;

/// Gets all tasks.
///
/// # Returns
/// A `Result` containing the response to the request or an error
pub async fn get_all() -> Result<impl IntoResponse, NanoServiceError> {
    let items = get_all_tasks()?;
    Ok((StatusCode::OK, Json(items)).into_response())
}
