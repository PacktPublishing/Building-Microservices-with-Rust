#[allow(dead_code)]
#[derive(Debug)]
/// A struct to represent a task
struct Task {
    /// The id of the task
    id: u32,
    /// The description of the task
    description: String,
    /// Whether the task is completed
    completed: bool,
}

fn main() {
    let task1 = Task {
        id: 1,
        description: String::from("Complete budget"),
        completed: false,
    };

    let task2 = task1;
    // println!("{:?}", task1);
    println!("Task: {}", task2.description);

    let mut task2 = Task {
        id: 1,
        description: String::from("Finish project"),
        completed: false,
    };
    let task2_desc = &mut task2.description;
    task2_desc.push_str(" by Friday");
    println!("Task: {}", task2.description);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let task = Task {
            id: 1,
            description: String::from("Complete budget"),
            completed: false,
        };

        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Complete budget");
        assert!(!task.completed);
    }

    #[test]
    fn test_task_ownership() {
        let task1 = Task {
            id: 1,
            description: String::from("Complete budget"),
            completed: false,
        };

        let task2 = task1;
        // assert_eq!(task1.description, "Complete budget"); // This will fail because task1 has been moved
        assert_eq!(task2.description, "Complete budget");
    }

    #[test]
    fn test_task_mutability() {
        let mut task2 = Task {
            id: 1,
            description: String::from("Finish project"),
            completed: false,
        };

        let task2_desc = &mut task2.description;
        task2_desc.push_str(" by Friday");
        assert_eq!(task2.description, "Finish project by Friday");
    }
}