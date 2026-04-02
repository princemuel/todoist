use actix_web::{HttpRequest, HttpResponse};
use shared::errors::{Error, ErrorStatus};
use shared::token::HeaderToken;
use task_core::actions::delete::delete as delete_core;
use task_core::actions::get::get_all as get_all_core;
use task_dal::tasks::transactions::delete::DeleteOne;
use task_dal::tasks::transactions::get::GetAll;

/// Deletes an item from the tasks list by name.
///
/// # Arguments
/// - `req` - The HTTP request.
///
/// # Returns
/// List of task items
pub async fn delete<T: DeleteOne + GetAll>(
    token: HeaderToken,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    match req.match_info().get("name") {
        Some(name) => {
            delete_core::<T>(name).await?;
        }
        None => {
            return Err(Error::new(
                "Resource name not provided".to_string(),
                ErrorStatus::BadRequest,
            ));
        }
    };

    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}
