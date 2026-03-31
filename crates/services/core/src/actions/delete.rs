#[cfg(feature = "json_fs")]
use dal::json::delete_one;
use shared::errors::Error;

use crate::models::Task;

pub fn delete(id: &str) -> Result<Task, Error> { delete_one(id) }
