use engine::actions::get::get_all as get_all_tasks;
use engine::models::Tasks;
use shared::errors::NanoServiceError;
use rocket::serde::json::Json;

/// Gets all tasks.
#[get("/tasks")]
pub async fn get_all() -> Result<Json<Tasks>, NanoServiceError> { Ok(Json(get_all_tasks()?)) }
