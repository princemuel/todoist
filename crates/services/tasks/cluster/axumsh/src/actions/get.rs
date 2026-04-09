use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use shared::errors::Error;
use shared::token::AuthToken;
use task_core::actions::get::{get_all as get_all_core, get_by_name as get_by_name_core};
use task_dal::tasks::transactions::get::{GetAll, GetByName};

/// Gets all tasks.
///
/// # Returns
/// A `Result` containing the response to the
/// request or an error
pub async fn get_all<T: GetAll>(_token: AuthToken) -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, Json(get_all_core::<T>().await?)))
}

/// Gets a task by name.
///
/// # Arguments
/// * `req`: The request the extract the name from the URL
///
/// # Returns
/// An `HttpResponse` with a JSON body containing of the task specified in the
/// URL
pub async fn get_by_name<T: GetByName>(
    _token: AuthToken,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, Json(get_by_name_core::<T>(&name).await?)))
}
