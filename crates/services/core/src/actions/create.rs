#[cfg(feature = "json_fs")]
use dal::json::create_one;
use shared::errors::SharedError;

use crate::models::Task;

pub fn create(item: Task) -> Result<Task, SharedError> {
    create_one(&item.title.clone(), &item)?;
    Ok(item)
}
