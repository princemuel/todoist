use actix_web::{HttpRequest, HttpResponse};
use task_dal::tasks::transactions::get::GetAll;
use task_core::actions::get::{get_all as get_all_core, get_by_name as get_by_name_core};
use shared::errors::{Error, ErrorStatus};

/// Gets all tasks.
///
/// # Returns
/// A `Result` containing the response to the request or an error
pub async fn get_all<T: GetAll>() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}

/// Gets a task by name.
///
/// # Arguments
/// * `req`: The request the extract the name from the URL
///
/// # Returns
/// An `HttpResponse` with a JSON body containing of the task specified in the
/// URL
pub async fn get_by_name<T: GetAll>(req: HttpRequest) -> Result<HttpResponse, Error> {
    let name = match req.match_info().get("name") {
        Some(name) => name,
        None => {
            return Err(Error::new(
                "Name not provided".to_string(),
                ErrorStatus::BadRequest,
            ));
        }
    };

    Ok(HttpResponse::Ok().json(get_by_name_core::<T>(name).await?))
}
