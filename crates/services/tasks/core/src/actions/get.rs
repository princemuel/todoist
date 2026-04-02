use shared::errors::Error;
use task_dal::tasks::schema::{Task, Tasks};
use task_dal::tasks::transactions::get::{GetAll, GetByName};

/// Gets all tasks.
///
/// # Errors
///
/// This function will return an error if saving to the db fails.
pub async fn get_all<T: GetAll>() -> Result<Tasks, Error> {
    Ok(Tasks::from(T::get_all().await?))
}

/// Gets a task by name.
///
/// # Errors
/// This function will return an error if saving to the db fails or if a
/// task with the provided name does not exist.
pub async fn get_by_name<T: GetByName>(name: &str) -> Result<Task, Error> {
    T::get_by_name(name).await
}
