use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use shared::errors::Error;
use shared::token::AuthToken;
use task_core::actions::get::get_all as get_all_core;
use task_core::actions::update::update as update_core;
use task_dal::tasks::schema::Task;
use task_dal::tasks::transactions::get::GetAll;
use task_dal::tasks::transactions::update::UpdateOne;

/// Updates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be updated.
///
/// # Returns
/// All the items in the task list
pub async fn update<T: UpdateOne + GetAll>(
    _token: AuthToken,
    Json(payload): Json<Task>,
) -> Result<impl IntoResponse, Error> {
    update_core::<T>(payload).await?;
    Ok((StatusCode::OK, Json(get_all_core::<T>().await?)))
}
