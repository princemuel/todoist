#[cfg(feature = "json")]
use std::collections::HashMap;

use shared::errors::Error;
#[cfg(any(feature = "json", feature = "sqlx-postgres"))]
use shared::errors::ErrorStatus;

#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::POSTGRES_POOL;
#[cfg(feature = "json")]
use crate::json::{create_many, find_many};
#[cfg(feature = "json")]
use crate::tasks::descriptors::JsonFileDescriptor;
#[cfg(feature = "sqlx-postgres")]
use crate::tasks::descriptors::SqlxPostgresDescriptor;
use crate::tasks::schema::Task;

pub trait DeleteOne {
    fn delete_one(title: String) -> impl Future<Output = Result<Task, Error>> + Send;
}
#[cfg(feature = "sqlx-postgres")]
impl DeleteOne for SqlxPostgresDescriptor {
    fn delete_one(title: String) -> impl Future<Output = Result<Task, Error>> + Send {
        sqlx_postgres_delete_one(title)
    }
}
#[cfg(feature = "json")]
impl DeleteOne for JsonFileDescriptor {
    fn delete_one(title: String) -> impl Future<Output = Result<Task, Error>> + Send {
        json_delete_one(title)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_delete_one(title: String) -> Result<Task, Error> {
    let item = sqlx::query_as("DELETE FROM tasks WHERE title = $1 RETURNING *")
        .bind(title)
        .fetch_one(&*POSTGRES_POOL)
        .await
        .map_err(|e| Error::new(e.to_string(), ErrorStatus::Unknown))?;
    Ok(item)
}

#[cfg(feature = "json")]
#[expect(clippy::unused_async)]
async fn json_delete_one(title: String) -> Result<Task, Error> {
    let mut items = find_many().unwrap_or_else(|_| HashMap::with_capacity(0));
    let item = items
        .remove(&title)
        .ok_or_else(|| Error::new("Item not found".to_owned(), ErrorStatus::NotFound))?;
    create_many(&items)?;
    Ok(item)
}
