use actix_web::HttpResponse;
use actix_web::web::Json;
use dal::tasks::schema::CreateTask;
use dal::tasks::transactions::create::SaveOne;
use dal::tasks::transactions::get::GetAll;
use engine::actions::create::create as create_one;
use engine::actions::get::get_all as get_all_core;
use shared::errors::Error;
use shared::token::HeaderToken;

/// Creates a task.
///
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
///
/// # Returns
/// All the items in the task list
pub async fn create<T: SaveOne + GetAll>(
    token: HeaderToken,
    payload: Json<CreateTask>,
) -> Result<HttpResponse, Error> {
    create_one::<T>(payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(get_all_core::<T>().await?))
}
