use actix_web::{HttpRequest, HttpResponse};
use engine::actions::delete::delete as delete_core;
use engine::actions::get::get_all as get_all_core;
use shared::errors::{Error, ErrorStatus};

/// Deletes an item from the tasks list by name.
///
/// # Arguments
/// - `req` - The HTTP request.
///
/// # Returns
/// List of task items
pub async fn delete(req: HttpRequest) -> Result<HttpResponse, Error> {
    match req.match_info().get("name") {
        Some(name) => {
            delete_core(name)?;
        }
        None => {
            return Err(Error::new(
                "Resource name not provided".to_string(),
                ErrorStatus::BadRequest,
            ));
        }
    };
    Ok(HttpResponse::Ok().json(get_all_core()?))
}
