use engine::actions::get::get_all as get_all_core;
use engine::actions::update::update as update_one;
use engine::models::{Task, Tasks};
use rocket::serde::json::Json;
use shared::errors::Error;
use shared::token::HeaderToken;

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
#[patch("/tasks", data = "<payload>")]
pub async fn update(token: HeaderToken, payload: Json<Task>) -> Result<Json<Tasks>, Error> {
    update_one(payload.into_inner())?;
    Ok(Json(get_all_core()?))
}
