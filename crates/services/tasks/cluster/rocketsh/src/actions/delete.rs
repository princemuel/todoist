use rocket::serde::json::Json;
use shared::errors::Error;
use shared::token::HeaderToken;
use task_core::actions::delete::delete as delete_core;
use task_core::actions::get::get_all as get_all_core;
use task_dal::tasks::descriptors::SqlxPostgresDescriptor;
use task_dal::tasks::schema::Tasks;

/// Deletes an item from the tasks list by name.
///
/// # Arguments
/// - `req` - The HTTP request.
///
/// # Returns
/// List of task items
#[delete("/tasks/<name>")]
pub async fn delete_by_name(token: HeaderToken, name: &str) -> Result<Json<Tasks>, Error> {
    delete_core::<SqlxPostgresDescriptor>(name).await?;
    Ok(Json(get_all_core::<SqlxPostgresDescriptor>().await?))
}
