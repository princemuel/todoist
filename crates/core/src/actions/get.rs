#[cfg(feature = "json_fs")]
use dal::json::find_many;

use crate::models::{Task, Tasks};

/// .
///
/// # Errors
///
/// This function will return an error if saving to the db fails.
pub fn get_all() -> Result<Tasks, String> {
    Ok(Tasks::from(find_many::<Task>().map_err(|e| e.to_string())?))
}
