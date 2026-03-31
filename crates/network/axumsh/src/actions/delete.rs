use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use engine::actions::delete::delete as delete_core;
use engine::actions::get::get_all as get_all_core;
use shared::errors::Error;

/// Deletes an item from the tasks list by name.
///
/// # Returns
/// List of task items
pub async fn delete(Path(name): Path<String>) -> Result<impl IntoResponse, Error> {
    delete_core(&name)?;

    Ok((StatusCode::OK, Json(get_all_core()?)))
}
