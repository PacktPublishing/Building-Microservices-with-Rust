use std::sync::Mutex;

#[allow(dead_code)]
#[derive(Debug)]
/// A struct to represent a task
pub struct Task {
    /// The id of the task
    pub id: u32,
    /// The description of the task
    pub description: String,
    /// Whether the task is completed
    pub completed: bool,
}

/// A struct to manage tasks
pub struct TaskManager {
    tasks: Mutex<Vec<Task>>,
}

impl Default for TaskManager {
    /// Creates a new TaskManager with default values
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    /// Creates a new TaskManager
    pub fn new() -> Self {
        TaskManager {
            tasks: Mutex::new(Vec::new()),
        }
    }

    /// Adds a task to the task manager
    pub fn add_task(&self, task: Task) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
    }

    /// Outputs all tasks to the console
    pub fn show_tasks(&self) {
        let tasks = self.tasks.lock().unwrap();
        for task in tasks.iter() {
            println!(
                "Task ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }
}
