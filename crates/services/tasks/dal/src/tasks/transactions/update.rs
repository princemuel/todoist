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

pub trait UpdateOne {
    fn update_one(item: Task) -> impl Future<Output = Result<Task, Error>> + Send;
}
#[cfg(feature = "sqlx-postgres")]
impl UpdateOne for SqlxPostgresDescriptor {
    fn update_one(item: Task) -> impl Future<Output = Result<Task, Error>> + Send {
        sqlx_postgres_update_one(item)
    }
}
#[cfg(feature = "json")]
impl UpdateOne for JsonFileDescriptor {
    fn update_one(item: Task) -> impl Future<Output = Result<Task, Error>> + Send {
        json_update_one(item)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_update_one(item: Task) -> Result<Task, Error> {
    let item =
        sqlx::query_as("UPDATE tasks SET title = $1, status = $2 WHERE id = $3 RETURNING *")
            .bind(item.title)
            .bind(item.status.clone())
            .bind(item.id)
            .fetch_one(&*POSTGRES_POOL)
            .await
            .map_err(|e| Error::new(e.to_string(), ErrorStatus::Unknown))?;
    Ok(item)
}

#[cfg(feature = "json")]
#[expect(clippy::unused_async)]
async fn json_update_one(item: Task) -> Result<Task, Error> {
    let mut items = find_many().unwrap_or_else(|_| HashMap::new());

    if !items.contains_key(&item.title) {
        return Err(Error::new(
            format!("Item with name {} not found", item.title),
            ErrorStatus::NotFound,
        ));
    }

    items.insert(item.title.clone(), item.clone());
    create_many(&items)?;
    Ok(item)
}
