use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::{env, io};

use serde::Serialize;
use serde::de::DeserializeOwned;

/// Opens a file.
///
/// # Arguments
/// - `path` - An optional string slice that specifies the path to the file.
///
/// # Returns
/// a file handle to perform read/write operations with.
fn file_handle(path: Option<&str>) -> io::Result<File> {
    let path = match path {
        Some(path) => path,
        None => &env::var("DATABASE_URL").unwrap_or_else(|_| "db.json".to_string()),
    };

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
}

/// Gets all the items from the JSON file.
///
/// # Returns
/// a hashmap of items.
pub fn find_many<T>() -> io::Result<HashMap<String, T>>
where
    T: DeserializeOwned,
{
    let mut f = file_handle(None)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let buffer = buffer.trim();
    if buffer.is_empty() {
        return Ok(HashMap::with_capacity(0));
    }

    let items = serde_json::from_str(buffer)?;
    Ok(items)
}

/// Gets an item from the JSON file.
///
/// # Arguments
/// - `id` - a string slice that specifies the id of the item.
///
/// # Returns
/// an item.
pub fn find_one<T>(id: &str) -> io::Result<T>
where
    T: DeserializeOwned + Clone,
{
    use io::{Error, ErrorKind};

    let items = find_many::<T>()?;
    match items.get(id) {
        None => Err(Error::new(
            ErrorKind::NotFound,
            format!("Resource with id '{id}' not found"),
        )),
        Some(item) => Ok(item.clone()),
    }
}

pub fn create_many<T>(items: &HashMap<String, T>) -> io::Result<()>
where
    T: Serialize,
{
    let mut file = file_handle(None)?;
    let json = serde_json::to_string_pretty(items)?;
    file.write_all(json.as_bytes())
}

/// Saves an item to the JSON file.
///
/// # Arguments
/// - `id` - a string slice that specifies the id of the item.
/// - `item` - a reference to the item to save.
pub fn create_one<T>(id: &str, item: &T) -> io::Result<()>
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
pub fn delete_one<T>(id: &str) -> io::Result<()>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut items = find_many::<T>().unwrap_or_else(|_| HashMap::with_capacity(0));
    items.remove(id);
    create_many(&items)
}

#[cfg(test)]
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
        println!("{:?}", tasks);

        let tasks = find_many::<i32>();
        println!("{:?}", tasks);
    }
}
