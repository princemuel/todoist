use shared::errors::Error;
use task_dal::tasks::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(id: &str) -> Result<(), Error> {
    T::delete_one(id.to_owned()).await?;
    Ok(())
}
