use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use shared::errors::Error;
use shared::token::AuthToken;
use task_core::actions::create::create as create_core;
use task_core::actions::get::get_all as get_all_core;
use task_dal::tasks::schema::CreateTask;
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
    _token: AuthToken,
    Json(payload): Json<CreateTask>,
) -> Result<impl IntoResponse, Error> {
    create_core::<T>(payload).await?;
    Ok((StatusCode::OK, Json(get_all_core::<T>().await?)))
}
