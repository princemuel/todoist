#[cfg(feature = "json_fs")]
use dal::json::find_many;
use shared::errors::SharedError;

use crate::models::{Task, Tasks};

/// .
///
/// # Errors
///
/// This function will return an error if saving to the db fails.
pub fn get_all() -> Result<Tasks, SharedError> { Ok(Tasks::from(find_many::<Task>()?)) }
