use core::fmt;

#[cfg(feature = "json_fs")]
use dal::json::create as save_one;

use crate::enums::TaskStatus;
use crate::models::done::Done;
use crate::models::pending::Pending;

/// This enum is a wrapper for the different item types supported for the create
/// API.
///
/// # Variants
/// - `Pending` - Represents a pending item.
/// - `Done` - Represents a done item.
#[derive(Debug, Clone)]
pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

impl fmt::Display for ItemTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending(status) => write!(f, "Pending: {}", status.parent.title),
            Self::Done(status) => write!(f, "Done: {}", status.parent.title),
        }
    }
}

/// This function creates a new item based on the title and status provided.
///
/// # Notes
/// This is the external interface for the create item API.
///
/// # Arguments
/// - `title` - The title of the item to be created.
/// - `status` - The status of the item to be created.
///
/// # Returns
/// An `ItemTypes` enum representing the item created.
/// # Errors
/// Returns a `String` error message if the creation fails.
pub fn create(title: &str, status: TaskStatus) -> Result<ItemTypes, String> {
    save_one(title, &status).map_err(|e| e.to_string())?;

    let item = match status {
        TaskStatus::Pending => ItemTypes::Pending(Pending::new(title)),
        TaskStatus::Done => ItemTypes::Done(Done::new(title)),
    };

    Ok(item)
}
