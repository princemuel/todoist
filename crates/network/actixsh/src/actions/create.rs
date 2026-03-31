use actix_web::HttpResponse;
use actix_web::web::Json;
use engine::actions::create::create as create_one;
use engine::actions::get::get_all as get_all_core;
use engine::models::Task;
use shared::errors::Error;

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
pub async fn create(payload: Json<Task>) -> Result<HttpResponse, Error> {
    create_one(payload.into_inner())?;
    Ok(HttpResponse::Ok().json(get_all_core()?))
}
