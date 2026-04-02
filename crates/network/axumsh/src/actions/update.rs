use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use task_core::actions::get::get_all as get_all_core;
use task_core::actions::update::update as update_one;
use task_core::models::Task;
use shared::errors::Error;
use shared::token::HeaderToken;

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
pub async fn update(
    token: HeaderToken,
    Json(payload): Json<Task>,
) -> Result<impl IntoResponse, Error> {
    update_one(payload)?;
    Ok((StatusCode::OK, Json(get_all_core()?)))
}
