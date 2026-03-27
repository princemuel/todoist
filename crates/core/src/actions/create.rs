#[cfg(feature = "json_fs")]
use dal::json::create_one;

use crate::enums::TaskStatus;
use crate::models::Task;

/// .
///
/// # Errors
///
/// This function will return an error if saving to the db fails.
pub fn create(title: &str, status: TaskStatus) -> Result<Task, String> {
    let item = Task {
        title: title.to_string(),
        status,
    };
    create_one(title, &status).map_err(|e| e.to_string())?;
    Ok(item)
}
