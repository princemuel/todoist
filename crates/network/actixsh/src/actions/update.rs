use actix_web::HttpResponse;
use actix_web::web::Json;
use engine::actions::get::get_all as get_all_core;
use engine::actions::update::update as update_core;
use engine::models::Task;
use shared::errors::Error;
use shared::token::HeaderToken;

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
pub async fn update(token: HeaderToken, payload: Json<Task>) -> Result<HttpResponse, Error> {
    update_core(payload.into_inner())?;
    Ok(HttpResponse::Ok().json(get_all_core()?))
}
