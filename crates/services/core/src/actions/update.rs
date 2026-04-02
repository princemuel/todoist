use dal::tasks::schema::Task;
use dal::tasks::transactions::update::UpdateOne;
use shared::errors::Error;

/// .
///
/// # Errors
///
/// This function will return an error if saving to the db fails.
pub async fn update<T: UpdateOne>(item: Task) -> Result<(), Error> {
    T::update_one(item).await?;
    Ok(())
}
