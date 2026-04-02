use actix_web::HttpResponse;
use actix_web::web::Json;
use dal::tasks::schema::Task;
use dal::tasks::transactions::get::GetAll;
use dal::tasks::transactions::update::UpdateOne;
use engine::actions::get::get_all as get_all_core;
use engine::actions::update::update as update_core;
use shared::errors::Error;
use shared::token::HeaderToken;

/// Updates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the updated task information.
///
/// # Returns
/// All the items in the task list
pub async fn update<T: UpdateOne + GetAll>(
    token: HeaderToken,
    payload: Json<Task>,
) -> Result<HttpResponse, Error> {
    update_core::<T>(payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}
