use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::status::TaskStatus;

/// A struct representing a task item
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    /// The title of the task
    pub title: String,
    /// The status of the task
    pub status: TaskStatus,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            TaskStatus::DONE => write!(f, "Done: {}", self.title),
            TaskStatus::PENDING => write!(f, "Pending: {}", self.title),
        }
    }
}

/// A struct representing all the task items.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tasks {
    /// A list of pending tasks
    pub pending: Vec<Task>,
    /// A list of done tasks
    pub done: Vec<Task>,
}

/// Converts a `HashMap` of tasks to a [`Tasks`]
/// struct.
///
/// # Arguments
/// - `items`: A [`HashMap`] of tasks.
///
/// # Returns
/// An [`Tasks`] struct.
impl From<HashMap<String, Task>> for Tasks {
    fn from(items: HashMap<String, Task>) -> Self {
        let (pending, done) = items
            .into_values()
            .partition(|item| matches!(item.status, TaskStatus::PENDING));

        Self { pending, done }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_displays() {
        let task = Task {
            title: "Test".to_owned(),
            status: TaskStatus::PENDING,
        };
        assert_eq!(format!("{task}"), "Pending: Test");
    }

    #[test]
    fn it_converts_from_hashmap() {
        let mut tasks = HashMap::with_capacity(2);
        tasks.insert("1".to_owned(), Task {
            title: "Test".to_owned(),
            status: TaskStatus::PENDING,
        });
        tasks.insert("2".to_owned(), Task {
            title: "Test".to_owned(),
            status: TaskStatus::DONE,
        });

        let tasks = Tasks::from(tasks);
        assert_eq!(tasks.pending.len(), 1);
        assert_eq!(tasks.done.len(), 1);
    }
}
