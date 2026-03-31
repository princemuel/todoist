#[cfg(feature = "json_fs")]
use dal::json::{create_many, find_many};
use shared::errors::{Error, ErrorStatus};

use crate::models::Task;

/// .
///
/// # Errors
///
/// This function will return an error if saving to the db fails.
pub fn update(item: Task) -> Result<(), Error> {
    let mut items = find_many()?;
    if !items.contains_key(&item.title) {
        return Err(Error::new(
            format!("Resource with name {} not found", item.title),
            ErrorStatus::NotFound,
        ));
    }

    items.insert(item.title.clone(), item);
    create_many(&items)
}
