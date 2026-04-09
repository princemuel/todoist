use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::errors::Error;
use shared::token::AuthToken;
use task_core::actions::create::create as create_one;
use task_core::actions::get::get_all as get_all_core;
use task_dal::tasks::descriptors::SqlxPostgresDescriptor;
use task_dal::tasks::schema::{CreateTask, Tasks};

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
#[post("/tasks", data = "<payload>")]
pub async fn create(
    _token: AuthToken,
    payload: Json<CreateTask>,
) -> Result<Created<Json<Tasks>>, Error> {
    create_one::<SqlxPostgresDescriptor>(payload.into_inner()).await?;
    let items = get_all_core::<SqlxPostgresDescriptor>().await?;
    Ok(Created::new("/tasks").body(Json(items)))
}
