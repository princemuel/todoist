use crate::enums::TaskStatus;

/// Base struct for the task.
///
/// # Notes
/// This base struct is referenced by other todo item structs representing the
/// core data that would be stored. The other structs referencing this struct
/// act as interfaces for the data held in this struct.
///
/// # Fields
/// * `title` - The title of the task
/// * `status` - The status of the task
#[derive(Clone, Debug)]
pub struct Base {
    pub title:  String,
    pub status: TaskStatus,
}
