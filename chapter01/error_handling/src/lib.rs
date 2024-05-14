#[allow(dead_code)]
#[derive(Debug)]
/// A struct to represent a task
pub struct Task {
    /// The id of the task
    pub id: u32,
    /// The description of the task
    pub description: String,
    /// Whether the task is completed
    completed: bool,
}

impl Task {
    /// Creates a new task with the given id and description
    pub fn new(id: u32, description: &str) -> Result<Task, String> {
        if description.is_empty() {
            return Err("Task description cannot be empty".to_string());
        }

        Ok(Task {
            id,
            description: description.to_string(),
            completed: false,
        })
    }
}
