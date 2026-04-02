use dal::tasks::transactions::delete::DeleteOne;
use shared::errors::Error;

pub async fn delete<T: DeleteOne>(id: &str) -> Result<(), Error> {
    T::delete_one(id.to_string()).await?;
    Ok(())
}
