#[cfg(feature = "json")]
use std::collections::HashMap;
use std::future::Future;

use shared::errors::Error;
#[cfg(feature = "sqlx-postgres")]
use shared::errors::ErrorStatus;

#[cfg(feature = "json")]
use super::super::descriptors::JsonFileDescriptor;
#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostgresDescriptor;
#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::POSTGRES_POOL;
#[cfg(feature = "json")]
use crate::json::find_many;
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
    let tasks = find_many().unwrap_or_else(|_| HashMap::new());
    let items = tasks.values().cloned().collect();
    Ok(items)
}
