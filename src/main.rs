use std::fs::File;
use std::io;
use std::io::{BufRead, Write};

struct Task {
    name: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();
    loop {
        print_menu();
        match read_command() {
            1 => tasks.append(&mut add_task()),
            2 => list_tasks(&tasks),
            3 => edit_task(get_index(&tasks), &mut tasks),
            4 => {
                save_tasks(&tasks);
                std::process::exit(0);
            }
            _ => println!("Invalid command"),
        }
    }
}

fn print_menu() {
    println!("\nTODO App");
    println!("1. ADD          | Add a new task");
    println!("2. LIST         | List all tasks");
    println!("3. DONE or EDIT | Mark a task as done or change its name");
    println!("4. EXIT         | Exit the program");
}

fn read_command() -> i32 {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    match command.trim() {
        "ADD" => 1,
        "LIST" => 2,
        "DONE" => 3,
        "EDIT" => 3,
        "EXIT" => 4,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        _ => 0,
    }
}

fn add_task() -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        let mut task_name = String::new();
        println!("Enter new task (press enter to stop adding):");
        io::stdin()
            .read_line(&mut task_name)
            .expect("Failed to read line");
        let task_name = task_name.trim();

        if task_name.is_empty() {
            break;
        }

        let task = Task {
            name: task_name.to_string(),
            done: false,
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
            println!("{}. [X] {}", i + 1, task.name);
        } else {
            println!("{}. [ ] {}", i + 1, task.name);
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
    println!("\nTask to edit: {} / Done: {}", tasks[task].name, tasks[task].done);
    print_edit_options();
    match read_edit_command() {
        1 => edit_task_name(task, tasks),
        2 => tasks[task].done = true,
        3 => (),
        _ => println!("Invalid command"),
    }
}

fn print_edit_options() {
    println!("1. EDIT | Edit task name");
    println!("2. DONE | Mark task as done");
    println!("3. BACK | Go back to main menu");
}

fn read_edit_command() -> i32 {
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    match command.trim() {
        "EDIT" => 1,
        "DONE" => 2,
        "BACK" => 3,
        "1" => 1,
        "2" => 2,
        "3" => 3,
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
        let line = format!("{} {}\n", task.name, task.done);
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
        let task = Task {
            name: name.to_string(),
            done,
        };
        tasks.push(task);
    }

    tasks
}