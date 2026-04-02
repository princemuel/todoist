#[cfg(feature = "json")]
use std::collections::HashMap;
use std::future::Future;

use shared::errors::Error;
#[cfg(feature = "sqlx-postgres")]
use shared::errors::ErrorStatus;

#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::POSTGRES_POOL;
#[cfg(feature = "json")]
use crate::json::find_many;
#[cfg(feature = "json")]
use crate::tasks::descriptors::JsonFileDescriptor;
#[cfg(feature = "sqlx-postgres")]
use crate::tasks::descriptors::SqlxPostgresDescriptor;
use crate::tasks::schema::Task;

pub trait GetAll {
    fn get_all() -> impl Future<Output = Result<Vec<Task>, Error>> + Send;
}
#[cfg(feature = "sqlx-postgres")]
impl GetAll for SqlxPostgresDescriptor {
    fn get_all() -> impl Future<Output = Result<Vec<Task>, Error>> + Send {
        sqlx_postgres_get_all()
    }
}
#[cfg(feature = "json")]
impl GetAll for JsonFileDescriptor {
    fn get_all() -> impl Future<Output = Result<Vec<Task>, Error>> + Send { json_get_all() }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_get_all() -> Result<Vec<Task>, Error> {
    let items = sqlx::query_as("SELECT * FROM tasks")
        .fetch_all(&*POSTGRES_POOL)
        .await
        .map_err(|e| Error::new(e.to_string(), ErrorStatus::Unknown))?;
    Ok(items)
}
#[cfg(feature = "json")]
#[allow(clippy::unused_async)]
async fn json_get_all() -> Result<Vec<Task>, Error> {
    let items = find_many().unwrap_or_else(|_| HashMap::with_capacity(0));
    let items = items.values().cloned().collect();
    Ok(items)
}

pub trait GetByName {
    fn get_by_name(name: &str) -> impl Future<Output = Result<Task, Error>> + Send;
}
#[cfg(feature = "sqlx-postgres")]
impl GetByName for SqlxPostgresDescriptor {
    fn get_by_name(name: &str) -> impl Future<Output = Result<Task, Error>> + Send {
        sqlx_postgres_get_by_name(name.to_string())
    }
}
#[cfg(feature = "json")]
impl GetByName for JsonFileDescriptor {
    fn get_by_name(name: &str) -> impl Future<Output = Result<Task, Error>> + Send {
        json_get_by_name(name.to_string())
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_get_by_name(name: String) -> Result<Task, Error> {
    let item = sqlx::query_as("SELECT * FROM tasks WHERE title = $1")
        .bind(name)
        .fetch_optional(&*POSTGRES_POOL)
        .await
        .map_err(|e| Error::new(e.to_string(), ErrorStatus::Unknown))?
        .ok_or_else(|| Error::new("Task not found".to_string(), ErrorStatus::NotFound))?;
    Ok(item)
}
#[cfg(feature = "json")]
#[allow(clippy::unused_async)]
async fn json_get_by_name(name: String) -> Result<Task, Error> {
    let items = find_many::<Task>().unwrap_or_else(|_| HashMap::with_capacity(0));
    items
        .values()
        .find(|task| task.title == name)
        .cloned()
        .ok_or_else(|| Error::new("Task not found".to_string(), ErrorStatus::NotFound))
}
