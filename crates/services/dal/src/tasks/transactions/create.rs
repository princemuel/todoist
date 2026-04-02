use core::future::Future;
#[cfg(feature = "json")]
use std::collections::HashMap;

use shared::errors::Error;
#[cfg(feature = "sqlx-postgres")]
use shared::errors::ErrorStatus;

#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::POSTGRES_POOL;
#[cfg(feature = "json")]
use crate::json::{create_many, find_many};
#[cfg(feature = "json")]
use crate::tasks::descriptors::JsonFileDescriptor;
#[cfg(feature = "sqlx-postgres")]
use crate::tasks::descriptors::SqlxPostgresDescriptor;
use crate::tasks::schema::{CreateTask, Task};

pub trait SaveOne {
    fn save_one(item: CreateTask) -> impl Future<Output = Result<Task, Error>> + Send;
}
#[cfg(feature = "sqlx-postgres")]
impl SaveOne for SqlxPostgresDescriptor {
    fn save_one(item: CreateTask) -> impl Future<Output = Result<Task, Error>> + Send {
        sqlx_postgres_save_one(item)
    }
}
#[cfg(feature = "json")]
impl SaveOne for JsonFileDescriptor {
    fn save_one(item: CreateTask) -> impl Future<Output = Result<Task, Error>> + Send {
        json_save_one(item)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_save_one(item: CreateTask) -> Result<Task, Error> {
    let item = sqlx::query_as("INSERT INTO items (title, status) VALUES ($1, $2) RETURNING *")
        .bind(item.title)
        .bind(item.status.to_string())
        .fetch_one(&*POSTGRES_POOL)
        .await
        .map_err(|e| Error::new(e.to_string(), ErrorStatus::Unknown))?;
    Ok(item)
}

#[cfg(feature = "json")]
#[allow(clippy::unused_async)]
async fn json_save_one(item: CreateTask) -> Result<Task, Error> {
    use uuid::Uuid;

    let mut items = find_many().unwrap_or_else(|_| HashMap::with_capacity(1));
    let item = Task {
        id: Uuid::now_v7(),
        public_id: Uuid::new_v4(),
        title: item.title,
        status: item.status.to_string(),
    };

    items.insert(item.title.clone(), item.clone());
    create_many(&items)?;

    Ok(item)
}
