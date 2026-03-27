use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::enums::TaskStatus;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub title:  String,
    pub status: TaskStatus,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            TaskStatus::Done => write!(f, "Done: {}", self.title),
            TaskStatus::Pending => write!(f, "Pending: {}", self.title),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tasks {
    pub pending: Vec<Task>,
    pub done:    Vec<Task>,
}

impl From<HashMap<String, Task>> for Tasks {
    fn from(items: HashMap<String, Task>) -> Self {
        let (pending, done) = items
            .into_values()
            .partition(|item| matches!(item.status, TaskStatus::Pending));

        Self { pending, done }
    }
}
