#[cfg(feature = "json_fs")]
use dal::json::create_one;
use shared::errors::Error;

use crate::models::Task;

pub fn create(item: Task) -> Result<Task, Error> {
    create_one(&item.title.clone(), &item)?;
    Ok(item)
}
