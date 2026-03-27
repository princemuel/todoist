#[cfg(feature = "json_fs")]
use dal::json::create_one;
use shared::errors::NanoServiceError;

use crate::models::Task;
use crate::status::TaskStatus;

/// .
///
/// # Errors
///
/// This function will return an error if saving to the db fails.
pub fn create(title: &str, status: TaskStatus) -> Result<Task, NanoServiceError> {
    let item = Task {
        title: title.to_string(),
        status,
    };
    create_one(title, &item)?;
    Ok(item)
}
