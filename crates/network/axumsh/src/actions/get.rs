use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use task_core::actions::get::{get_all as get_all_core, get_by_name as get_by_name_core};
use shared::errors::Error;

/// Gets all tasks.
///
/// # Returns
/// A `Result` containing the response to the
/// request or an error
pub async fn get_all() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, Json(get_all_core()?)))
}

/// Gets a task by name.
///
/// # Arguments
/// * `req`: The request the extract the name from the URL
///
/// # Returns
/// An `HttpResponse` with a JSON body containing of the task specified in the
/// URL
pub async fn get_by_name(Path(name): Path<String>) -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, Json(get_by_name_core(&name)?)))
}
