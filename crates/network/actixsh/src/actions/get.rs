use actix_web::HttpResponse;
use engine::actions::get::get_all as get_all_tasks;
use shared::errors::NanoServiceError;

/// Gets all tasks.
///
/// # Returns
/// A `Result` containing the response to the request or an error
pub async fn get_all() -> Result<HttpResponse, NanoServiceError> {
    Ok(HttpResponse::Ok().json(get_all_tasks()?))
}
