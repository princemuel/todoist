use crate::enums::TaskStatus;
use crate::models::base::Base;

/// Struct for pending tasks.
///
/// # Fields
/// * `parent` - The base struct for the task pointing to the data that can be
///   stored.
#[derive(Clone, Debug)]
pub struct Pending {
    pub parent: Base,
}

impl Pending {
    /// The constructor for the pending task.
    ///
    /// # Arguments
    /// * `title` - The title of the task
    ///
    /// # Returns
    /// A new instance of the pending task
    #[must_use]
    pub fn new(title: &str) -> Self {
        let base = Base {
            title:  title.to_string(),
            status: TaskStatus::Pending,
        };

        Self { parent: base }
    }
}
