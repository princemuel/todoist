use task_dal::tasks::schema::{Task, Tasks};
use task_dal::tasks::transactions::get::GetAll;
use shared::errors::{Error, ErrorStatus};

/// .
///
/// # Errors
///
/// This function will return an error if saving to the db fails.
pub async fn get_all<T: GetAll>() -> Result<Tasks, Error> {
    Ok(Tasks::from(T::get_all().await?))
}

pub async fn get_by_name<T: GetAll>(name: &str) -> Result<Task, Error> {
    let tasks = T::get_all().await?;
    tasks
        .into_iter()
        .find(|task| task.title == name)
        .ok_or_else(|| {
            Error::new(
                format!("Resource with name {name} not found"),
                ErrorStatus::NotFound,
            )
        })
}
