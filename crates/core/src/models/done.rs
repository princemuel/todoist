use crate::enums::TaskStatus;
use crate::models::base::Base;

/// Struct for done tasks.
///
/// # Fields
/// * `parent` - The base struct for the task pointing to the data that can be
///   stored.
#[derive(Clone, Debug)]
pub struct Done {
    pub parent: Base,
}

impl Done {
    /// The constructor for the done task.
    ///
    /// # Arguments
    /// * `title` - The title of the task
    ///
    /// # Returns
    /// A new instance of the done task
    #[must_use]
    pub fn new(title: &str) -> Self {
        let base = Base {
            title:  title.to_string(),
            status: TaskStatus::Done,
        };

        Self { parent: base }
    }
}
