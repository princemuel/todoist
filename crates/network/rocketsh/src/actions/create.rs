use task_core::actions::create::create as create_one;
use task_core::actions::get::get_all as get_all_core;
use task_core::models::{Task, Tasks};
use rocket::serde::json::Json;
use shared::errors::Error;

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
#[post("/tasks", data = "<payload>")]
pub async fn create(payload: Json<Task>) -> Result<Json<Tasks>, Error> {
    create_one(payload.into_inner())?;
    Ok(Json(get_all_core()?))
}
