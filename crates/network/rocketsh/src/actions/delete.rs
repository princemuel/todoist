use engine::actions::delete::delete as delete_core;
use engine::actions::get::get_all as get_all_core;
use engine::models::Tasks;
use rocket::serde::json::Json;
use shared::errors::Error;

/// Deletes an item from the tasks list by name.
///
/// # Arguments
/// - `req` - The HTTP request.
///
/// # Returns
/// List of task items
#[delete("/tasks/<name>")]
pub async fn delete_by_name(name: &str) -> Result<Json<Tasks>, Error> {
    delete_core(name)?;
    Ok(Json(get_all_core()?))
}
