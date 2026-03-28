use core::hash::BuildHasher;
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use serde::Serialize;
use serde::de::DeserializeOwned;
use shared::errors::{SharedError, SharedErrorStatus};
use shared::safe_eject;

/// Opens a file.
///
/// # Arguments
/// - `path` - An optional string slice that specifies the path to the file.
///
/// # Returns
/// a file handle to perform read/write operations with.
///
/// # Errors
/// Returns a [`SharedError`] if the file cannot be opened or created.
fn file_handle(path: Option<&str>) -> Result<File, SharedError> {
    let path = match path {
        Some(path) => path,
        None => &env::var("DATABASE_URL").unwrap_or_else(|_| "db.local.json".to_string()),
    };

    safe_eject!(
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path),
        SharedErrorStatus::Unknown,
        "Error writing resource to database"
    )
}

/// Gets all the items from the JSON file.
///
/// # Returns
/// a hashmap of items.
///
/// # Errors
/// Returns an [`SharedError`] if the file cannot be read or if the JSON is
/// malformed.
pub fn find_many<T>() -> Result<HashMap<String, T>, SharedError>
where
    T: DeserializeOwned,
{
    let mut f = file_handle(None)?;
    let mut buffer = String::new();

    safe_eject!(
        f.read_to_string(&mut buffer),
        SharedErrorStatus::Unknown,
        "Error reading database"
    )?;

    let buffer = buffer.trim();
    if buffer.is_empty() {
        return Ok(HashMap::with_capacity(0));
    }

    let items = safe_eject!(serde_json::from_str(buffer), SharedErrorStatus::Unknown)?;

    Ok(items)
}

/// Gets an item from the JSON file.
///
/// # Arguments
/// - `id` - a string slice that specifies the id of the item.
///
/// # Returns
/// an item.
///
/// # Errors
/// Returns an [`SharedError`] with [`SharedErrorStatus::NotFound`] if
/// no item with the given `id` exists.
pub fn find_one<T>(id: &str) -> Result<T, SharedError>
where
    T: DeserializeOwned + Clone,
{
    let items = find_many::<T>()?;
    match items.get(id) {
        None => Err(SharedError::new(
            format!("Resource with id '{id}' not found"),
            SharedErrorStatus::NotFound,
        )),
        Some(item) => Ok(item.clone()),
    }
}

/// # Errors
/// Returns an [`SharedError`] if the file cannot be written or if
/// serialization fails.
pub fn create_many<T, S>(items: &HashMap<String, T, S>) -> Result<(), SharedError>
where
    T: Serialize,
    S: BuildHasher,
{
    let mut file = file_handle(None)?;

    let body = safe_eject!(
        serde_json::to_string_pretty(items),
        SharedErrorStatus::Unknown,
        "Error serializing JSON before saving tasks"
    )?;

    safe_eject!(
        file.write_all(body.as_bytes()),
        SharedErrorStatus::Unknown,
        "Error writing tasks to JSON to file"
    )
}

/// Saves an item to the JSON file.
///
/// # Arguments
/// - `id` - a string slice that specifies the id of the item.
/// - `item` - a reference to the item to save.
///
/// # Errors
/// Returns an [`SharedError`] if reading, serializing, or writing to the
/// file fails.
pub fn create_one<T>(id: &str, item: &T) -> Result<(), SharedError>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut items = find_many::<T>().unwrap_or_else(|_| HashMap::with_capacity(1));
    items.insert(id.to_string(), item.clone());
    create_many(&items)
}

/// Deletes an item from the JSON file.
///
/// # Arguments
/// - `id` - a string slice that specifies the id of the item to delete.
///
/// # Errors
/// Returns an [`SharedError`] if reading, serializing, or writing to the
/// file fails.
pub fn delete_one<T>(id: &str) -> Result<T, SharedError>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut items = find_many::<T>().unwrap_or_else(|_| HashMap::with_capacity(0));
    match items.remove(id) {
        Some(item) => {
            create_many(&items)?;
            Ok(item)
        }
        None => Err(SharedError::new(
            format!("Resource with id {id} not found",),
            SharedErrorStatus::NotFound,
        )),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {

    use super::*;

    #[test]
    #[cfg(feature = "json")]
    fn test_file_handle() {
        let file = file_handle(None);
        assert!(file.is_ok());

        let file = file_handle(Some("./non_existent_file.json"));
        assert!(file.is_ok());
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_find_many() {
        create_one("1", &"Task 1".to_string()).unwrap();

        let tasks = find_many::<String>().unwrap();
        println!("{tasks:?}");

        let tasks = find_many::<i32>();
        println!("{tasks:?}");
    }
}
