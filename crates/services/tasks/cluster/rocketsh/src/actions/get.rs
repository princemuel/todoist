use rocket::serde::json::Json;
use shared::errors::Error;
use task_core::actions::get::{get_all as get_all_core, get_by_name as get_by_name_core};
use task_core::models::{Task, Tasks};

/// Gets all tasks.
///
/// # Returns
/// A `Result` containing the response to the
/// request or an error
#[get("/tasks")]
pub async fn get_all() -> Result<Json<Tasks>, Error> { Ok(Json(get_all_core()?)) }

/// Gets a task by name.
#[get("/tasks/<name>")]
pub async fn get_by_name(name: &str) -> Result<Json<Task>, Error> {
    Ok(Json(get_by_name_core(name)?))
}
