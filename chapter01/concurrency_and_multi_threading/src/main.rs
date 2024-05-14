use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use concurrency_and_multi_threading::{Task, TaskManager};

fn main() {
    // Threads
    let handle_task1 = thread::spawn(|| {
        // Simulate a long-running task
        thread::sleep(std::time::Duration::from_secs(5));
        println!("Long-running task completed");
    });
    // Continue executing other tasks
    println!("Executing other tasks...");
    // Wait for the spawned thread to finish
    handle_task1.join().unwrap();

    // Message passing
    let (tx, rx) = mpsc::channel();

    let handle_task2 = thread::spawn(move || {
        let task = Task {
            id: 1,
            description: "Complete budget".to_string(),
            completed: false,
        };
        tx.send(task).unwrap();
    });

    let received_task = rx.recv().unwrap();
    println!("Received task: {}", received_task.description);
    handle_task2.join().unwrap();

    // Shared mutable state
    let task_manager = Arc::new(TaskManager::new());

    let manager_clone = task_manager.clone();
    thread::spawn(move || {
        let task = Task {
            id: 1,
            description: "Complete budget".to_string(),
            completed: false,
        };
        manager_clone.add_task(task);
    });

    let task = Task {
        id: 2,
        description: "Prepare presentation".to_string(),
        completed: false,
    };
    task_manager.add_task(task);

    task_manager.show_tasks();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_long_running_task() {
        let handle_task1 = thread::spawn(|| {
            // Simulate a long-running task
            thread::sleep(Duration::from_secs(1));
            println!("Long-running task completed");
        });

        // Continue executing other tasks
        println!("Executing other tasks...");
        handle_task1.join().unwrap();
    }

    #[test]
    fn test_message_passing() {
        let (tx, rx) = mpsc::channel();

        let handle_task2 = thread::spawn(move || {
            let task = Task {
                id: 1,
                description: "Complete budget".to_string(),
                completed: false,
            };
            tx.send(task).unwrap();
        });

        let received_task = rx.recv().unwrap();
        assert_eq!(received_task.description, "Complete budget");
        handle_task2.join().unwrap();
    }

    #[test]
    fn test_shared_mutable_state() {
        let task_manager = Arc::new(TaskManager::new());

        let manager_clone = task_manager.clone();
        let handle_task3 = thread::spawn(move || {
            let task = Task {
                id: 1,
                description: "Complete budget".to_string(),
                completed: false,
            };
            manager_clone.add_task(task);
        });

        let task = Task {
            id: 2,
            description: "Prepare presentation".to_string(),
            completed: false,
        };
        task_manager.add_task(task);

        handle_task3.join().unwrap();
        task_manager.show_tasks();
    }
}
