#[cfg(feature = "json_fs")]
use dal::json::find_many;
use shared::errors::{Error, ErrorStatus};

use crate::models::{Task, Tasks};

/// .
///
/// # Errors
///
/// This function will return an error if saving to the db fails.
pub fn get_all() -> Result<Tasks, Error> { Ok(Tasks::from(find_many::<Task>()?)) }

pub fn get_by_name(name: &str) -> Result<Task, Error> {
    find_many()?.remove(name).ok_or_else(|| {
        Error::new(
            format!("Resource with name {name} not found"),
            ErrorStatus::NotFound,
        )
    })
}
