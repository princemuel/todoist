#[cfg(feature = "json_fs")]
use dal::json::delete_one;
use shared::errors::SharedError;

use crate::models::Task;

pub fn delete(id: &str) -> Result<Task, SharedError> { delete_one(id) }
