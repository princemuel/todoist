use task_dal::tasks::schema::{CreateTask, Task};
use task_dal::tasks::transactions::create::SaveOne;
use shared::errors::Error;

pub async fn create<T: SaveOne>(item: CreateTask) -> Result<Task, Error> {
    let item = T::save_one(item).await?;
    Ok(item)
}
