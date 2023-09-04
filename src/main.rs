use std::fs::File;
use std::io;
use std::io::{BufRead, Write};

struct Task {
    name: String,
    done: bool,
    tags: Vec<String>,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();
    loop {
        print_menu();
        let (action, index) = read_command();
        match action {
            1 => tasks.append(&mut add_task()),
            2 => list_tasks(&tasks),
            3 => {
                if index > 0 && index <= tasks.len() {
                    edit_task(index - 1, &mut tasks);
                } else if index == 0 {
                    let index = get_index(&tasks);
                    edit_task(index, &mut tasks);
                } else {
                    println!("Invalid index");
                }
            }
            4 => {
                if index > 0 && index <= tasks.len() {
                    mark_task_done(index, &mut tasks);
                } else {
                    println!("Invalid index");
                }
            }
            5 => {
                save_tasks(&tasks);
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}

fn print_menu() {
    println!("\nTODO App");
    println!("ADD    | Add a new task");
    println!("LIST   | List all tasks");
    println!("EDIT   | Edit a task");
    println!("EDIT 1 | Edit task number 1 (replace 1 with the task number)");
    println!("DONE 1 | Mark task number 1 as done (replace 1 with the task number)");
    println!("EXIT   | Exit the program");
}

fn read_command() -> (i32, usize) {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    let action = match parts.get(0) {
        Some(&action) => action,
        None => "",
    };
    let index = match parts.get(1) {
        Some(&index) => match index.parse() {
            Ok(index) => index,
            Err(_) => 0,
        },
        None => 0,
    };
    match action {
        "ADD" => (1, 0),
        "LIST" => (2, 0),
        "EDIT" => (3, index),
        "DONE" => (4, index),
        "EXIT" => (5, 0),
        _ => (0, 0),
    }
}


fn add_task() -> Vec<Task> {
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

fn list_tasks(tasks: &Vec<Task>) {
    println!("\nTasks:");
    for (i, task) in tasks.iter().enumerate() {
        if task.done {
            println!("{}. [X] {} - Tags: {:?}", i + 1, task.name, task.tags);
        } else {
            println!("{}. [ ] {} - Tags: {:?}", i + 1, task.name, task.tags);
        }
    }
}

fn get_index(tasks: &Vec<Task>) -> usize {
    list_tasks(tasks);
    loop {
        let mut index = String::new();
        println!("Enter task number to edit:");
        io::stdin().read_line(&mut index).expect("Failed to read line");
        let index: usize = match index.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid index");
                continue;
            }
        };
        if index > 0 && index <= tasks.len() {
            return index - 1;
        } else {
            println!("Invalid index");
        }
    }
}

fn edit_task(task: usize, tasks: &mut Vec<Task>) {
    println!("\nTask to edit: {} / Done: {} - Tags: {:?}", tasks[task].name, tasks[task].done, tasks[task].tags);
    print_edit_options();
    match read_edit_command() {
        1 => edit_task_name(task, tasks),
        2 => tasks[task].done = true,
        3 => {
            println!("Enter new tags (comma-separated):");
            let mut tags_input = String::new();
            io::stdin()
                .read_line(&mut tags_input)
                .expect("Failed to read line");
            let tags: Vec<String> = tags_input
                .split(',')
                .map(|tag| tag.trim().to_string())
                .collect();
            tasks[task].tags = tags;
        }
        4 => (),
        _ => println!("Invalid command"),
    }
}

fn print_edit_options() {
    println!("EDIT | Edit task name");
    println!("DONE | Mark task as done");
    println!("TAGS | Edit task tags");
    println!("BACK | Go back to main menu");
}

fn read_edit_command() -> i32 {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    match command.trim() {
        "EDIT" => 1,
        "DONE" => 2,
        "TAGS" => 3,
        "BACK" => 4,
        _ => 0,
    }
}

fn edit_task_name(index: usize, tasks: &mut Vec<Task>) {
    println!("Enter new task name:");
    let mut task_name = String::new();
    io::stdin().read_line(&mut task_name).expect("Failed to read line");
    tasks[index].name = task_name.trim().to_string();
}

fn save_tasks(tasks: &Vec<Task>) {
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

fn load_tasks() -> Vec<Task> {
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

fn mark_task_done(index: usize, tasks: &mut Vec<Task>) {
    if let Some(task) = tasks.get_mut(index - 1) {
        task.done = true;
        println!("Task number {} marked as done", index);
    } else {
        println!("Invalid index");
    }
}
