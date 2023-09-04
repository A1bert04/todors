use std::fs::File;
use std::io::{BufRead, Write};
use crate::Task;

pub fn load_tasks() -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();

    let file = match File::open("tasks.txt") {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file: {}", err);
            return tasks;
        }
    };

    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                println!("Failed to read line: {}", err);
                return tasks;
            }
        };
        let mut parts = line.split_whitespace();
        let name = match parts.next() {
            Some(name) => name,
            None => {
                println!("Failed to read task name");
                return tasks;
            }
        };
        let done = match parts.next() {
            Some(done) => match done.parse() {
                Ok(done) => done,
                Err(err) => {
                    println!("Failed to parse task done: {}", err);
                    return tasks;
                }
            },
            None => {
                println!("Failed to read task done");
                return tasks;
            }
        };
        let tags = match parts.next() {
            Some(tags) => tags.split(',').map(|tag| tag.to_string()).collect(),
            None => {
                println!("Failed to read task tags");
                return tasks;
            }
        };
        let task = Task {
            name: name.to_string(),
            done,
            tags,
        };
        tasks.push(task);
    }

    tasks
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let mut file = match File::create("tasks.txt") {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to create file: {}", err);
            return;
        }
    };

    for task in tasks {
        let line = format!("{} {} {}\n", task.name, task.done, task.tags.join(","));
        if let Err(err) = file.write_all(line.as_bytes()) {
            println!("Failed to write to file: {}", err);
            return;
        }
    }

    println!("Tasks saved to file.");
}