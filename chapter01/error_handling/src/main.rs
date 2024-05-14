use error_handling::Task;

fn main() {
    // Create a new task, using match to handle error case
    let task1 = Task::new(1, "Complete budget");
    match task1 {
        Ok(task) => println!("Created task: {}", task.description),
        Err(err) => println!("Error creating task: {}", err),
    }

    // Create another task, using match to handle error case
    let task2 = Task::new(2, "");
    match task2 {
        Ok(task) => println!("Created task: {}", task.description),
        Err(err) => println!("Error creating task: {}", err),
    }

    // Create another task, this time using unwrap to handle error case
    let task1 = Task::new(1, "Complete budget").unwrap();
    println!("Created task: {}", task1.description);

    // Create another task, this time using expect to handle error case
    let task2 = Task::new(2, "").expect("Task creation failed");
    println!("Created task: {}", task2.description);
}

#[cfg(test)]
mod tests {
    use error_handling::Task;

    #[test]
    fn test_create_task_success() {
        let task: Result<Task, String> = Task::new(1, "Complete budget");
        match task {
            Ok(t) => {
                assert_eq!(t.id, 1);
                assert_eq!(t.description, "Complete budget");
            }
            Err(err) => panic!("Error creating task: {}", err),
        }
    }

    #[test]
    fn test_create_task_error() {
        let task = Task::new(2, "");
        match task {
            Ok(_) => panic!("Task creation should have failed"),
            Err(err) => assert_eq!(err, "Task description cannot be empty"),
        }
    }

    #[test]
    fn test_create_task_unwrap() {
        let task = Task::new(1, "Complete budget").unwrap();
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Complete budget");
    }

    #[test]
    #[should_panic(expected = "Task description cannot be empty")]
    fn test_create_task_expect() {
        let _task = Task::new(2, "").expect("Task creation failed");
    }
}