use std::io;
use crate::Task;

pub fn add_task() -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        println!("Enter new task (press enter to stop adding):");
        let mut task_name = String::new();
        io::stdin()
            .read_line(&mut task_name)
            .expect("Failed to read line");
        let task_name = task_name.trim();

        if task_name.is_empty() {
            break;
        }

        println!("Enter tags for the task (comma-separated):");
        let mut tags_input = String::new();
        io::stdin()
            .read_line(&mut tags_input)
            .expect("Failed to read line");
        let tags: Vec<String> = tags_input
            .split(',')
            .map(|tag| tag.trim().to_string())
            .collect();

        let task = Task {
            name: task_name.to_string(),
            done: false,
            tags,
        };
        tasks.push(task);

        println!("Task added: {}", task_name);
    }

    tasks
}
