use serde::{Deserialize, Serialize};

use crate::tasks::enums::TaskStatus;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub status: TaskStatus,
}

use uuid::Uuid;
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub status: String,
}

/// A struct representing all the task items.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tasks {
    /// A list of pending tasks
    pub pending: Vec<Task>,
    /// A list of done tasks
    pub done: Vec<Task>,
}

/// Converts a `Vec` of tasks to a [`Tasks`] struct.
///
/// # Arguments
/// - `items`: A [`Vec`] of tasks.
///
/// # Returns
/// An [`Tasks`] struct.
impl From<Vec<Task>> for Tasks {
    fn from(items: Vec<Task>) -> Self {
        let (pending, done) = items
            .into_iter()
            .map(|item| Task {
                status: item.status.to_uppercase(),
                ..item
            })
            .partition(|item| matches!(item.status.as_str(), "PENDING"));

        Self { pending, done }
    }
}
