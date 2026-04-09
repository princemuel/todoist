use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};
use shared::errors::{Error, ErrorStatus};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    #[must_use]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::PENDING => "PENDING",
            Self::DONE => "DONE",
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}

impl FromStr for TaskStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(Error::new(
                "invalid status".to_owned(),
                ErrorStatus::BadRequest,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_status() {
        assert_eq!(TaskStatus::DONE.to_string(), "done");
        assert_eq!(TaskStatus::PENDING.to_string(), "pending");

        let done = format!("{}", TaskStatus::DONE);
        let pending = format!("{}", TaskStatus::PENDING);

        assert_eq!(done, "DONE");
        assert_eq!(pending, "PENDING");

        let done = TaskStatus::DONE.to_string();
        let pending = TaskStatus::PENDING.to_string();

        assert_eq!(done, "DONE");
        assert_eq!(pending, "PENDING");
    }

    #[test]
    fn task_status_from_string() {
        let done = "Done".to_owned();
        let pending = "Pending".to_owned();
        let invalid = "INVALID".to_owned();

        assert_eq!(pending.parse::<TaskStatus>().unwrap(), TaskStatus::PENDING);
        assert_eq!(done.parse::<TaskStatus>().unwrap(), TaskStatus::DONE);
        assert!(invalid.parse::<TaskStatus>().is_err());
    }
}
