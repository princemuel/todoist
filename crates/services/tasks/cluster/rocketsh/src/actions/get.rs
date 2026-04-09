use rocket::serde::json::Json;
use shared::errors::Error;
use shared::token::AuthToken;
use task_core::actions::get::{get_all as get_all_core, get_by_name as get_by_name_core};
use task_dal::tasks::descriptors::SqlxPostgresDescriptor;
use task_dal::tasks::schema::{Task, Tasks};

/// Gets all tasks.
///
/// # Returns
/// A `Result` containing the response to the
/// request or an error
#[get("/tasks")]
pub async fn get_all(_token: AuthToken) -> Result<Json<Tasks>, Error> {
    Ok(Json(get_all_core::<SqlxPostgresDescriptor>().await?))
}

/// Gets a task by name.
#[get("/tasks/<name>")]
pub async fn get_by_name(_token: AuthToken, name: &str) -> Result<Json<Task>, Error> {
    Ok(Json(
        get_by_name_core::<SqlxPostgresDescriptor>(name).await?,
    ))
}
