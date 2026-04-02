use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use shared::errors::Error;
use task_core::actions::delete::delete as delete_core;
use task_core::actions::get::get_all as get_all_core;
use task_dal::tasks::transactions::delete::DeleteOne;
use task_dal::tasks::transactions::get::GetAll;

/// Deletes an item from the tasks list by name.
///
/// # Returns
/// List of task items
pub async fn delete<T: DeleteOne + GetAll>(
    Path(name): Path<String>,
) -> Result<impl IntoResponse, Error> {
    delete_core::<T>(&name).await?;
    Ok((StatusCode::OK, Json(get_all_core::<T>().await?)))
}
