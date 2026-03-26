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
