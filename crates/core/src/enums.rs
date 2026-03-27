use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum TaskStatus {
    Done,
    #[default]
    Pending,
}

impl TaskStatus {
    #[must_use]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Done => "done",
            Self::Pending => "pending",
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = self.as_str();
        write!(f, "{status}")
    }
}

impl FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "done" => Ok(Self::Done),
            _ => Err(format!("Invalid Status: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status() {
        assert_eq!(TaskStatus::Done.to_string(), "Done");
        assert_eq!(TaskStatus::Pending.to_string(), "Pending");

        let done = format!("{}", TaskStatus::Done);
        let pending = format!("{}", TaskStatus::Pending);

        assert_eq!(done, "Done");
        assert_eq!(pending, "Pending");

        let done = TaskStatus::Done.to_string();
        let pending = TaskStatus::Pending.to_string();

        assert_eq!(done, "Done");
        assert_eq!(pending, "Pending");
    }

    #[test]
    fn test_task_status_from_string() {
        let done = "Done".to_string();
        let pending = "Pending".to_string();
        let invalid = "INVALID".to_string();

        assert_eq!(pending.parse::<TaskStatus>().unwrap(), TaskStatus::Pending);
        assert_eq!(done.parse::<TaskStatus>().unwrap(), TaskStatus::Done);
        assert_eq!(invalid.parse::<TaskStatus>().is_err(), true);
    }
}
