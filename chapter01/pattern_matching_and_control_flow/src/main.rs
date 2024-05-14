#[allow(dead_code)]
#[derive(Debug)]
/// A struct to represent a task
struct Task {
    /// The id of the task
    id: u32,
    /// The description of the task
    description: String,
    /// The status of the task
    status: TaskStatus,
}

#[allow(dead_code)]
#[derive(Debug)]
/// Represents the different states a task may be in
#[derive(PartialEq)]
enum TaskStatus {
    Todo,
    InProgress,
    Completed,
}

fn main() {
    let task1 = Task {
        id: 1,
        description: String::from("Complete budget"),
        status: TaskStatus::Todo,
    };

    match task1.status {
        TaskStatus::Todo => println!("Task `{}` is pending", task1.description),
        TaskStatus::InProgress => println!("Task `{}` is in progress", task1.description),
        TaskStatus::Completed => println!("Task `{}` is completed", task1.description),
    }

    let optional_task1: Option<Task> = Some(Task {
        id: 1,
        description: String::from("Prepare monthly report "),
        status: TaskStatus::Todo,
    });

    if let Some(task) = optional_task1 {
        println!("Task: {} (Status: {:?})", task.description, task.status);
    } else {
        println!("No task assigned!");
    }

    let optional_task2: Option<Task> = None;
    if let Some(task) = optional_task2 {
        println!("Task: {} (Status: {:?})", task.description, task.status);
    } else {
        println!("No task assigned!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let task = Task {
            id: 1,
            description: String::from("Complete budget"),
            status: TaskStatus::Todo,
        };

        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Complete budget");
        assert_eq!(task.status, TaskStatus::Todo);
    }

    #[test]
    fn test_task_status_match() {
        let task1 = Task {
            id: 1,
            description: String::from("Complete budget"),
            status: TaskStatus::Todo,
        };

        let mut output = String::new();
        match task1.status {
            TaskStatus::Todo => output.push_str("Task `Complete budget` is pending"),
            TaskStatus::InProgress => output.push_str("Task `Complete budget` is in progress"),
            TaskStatus::Completed => output.push_str("Task `Complete budget` is completed"),
        }

        assert_eq!(output, "Task `Complete budget` is pending");
    }

    #[test]
    fn test_optional_task() {
        let optional_task1: Option<Task> = Some(Task {
            id: 1,
            description: String::from("Prepare monthly report "),
            status: TaskStatus::Todo,
        });

        if let Some(task) = optional_task1 {
            assert_eq!(task.description, "Prepare monthly report ");
            assert_eq!(task.status, TaskStatus::Todo);
        } else {
            panic!("Optional task should not be None");
        }

        let optional_task2: Option<Task> = None;
        if let Some(_task) = optional_task2 {
            panic!("Optional task should be None");
        }
    }
}
