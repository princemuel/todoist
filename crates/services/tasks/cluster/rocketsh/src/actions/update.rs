use rocket::serde::json::Json;
use shared::errors::Error;
use shared::token::AuthToken;
use task_core::actions::get::get_all as get_all_core;
use task_core::actions::update::update as update_core;
use task_dal::tasks::descriptors::SqlxPostgresDescriptor;
use task_dal::tasks::schema::{Task, Tasks};

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
#[patch("/tasks", data = "<payload>")]
pub async fn update(_token: AuthToken, payload: Json<Task>) -> Result<Json<Tasks>, Error> {
    update_core::<SqlxPostgresDescriptor>(payload.into_inner()).await?;
    Ok(Json(get_all_core::<SqlxPostgresDescriptor>().await?))
}
