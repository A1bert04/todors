use std::io;
use crate::create::add_task;

mod files;
mod edit;
mod create;
use crate::edit::{edit_task, mark_task_done};
use crate::files::{load_tasks, save_tasks};


pub struct Task {
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
            5 => search(&tasks),
            6 => {
                save_tasks(&tasks);
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}

fn print_menu() {
    println!("\nTODO App");
    println!("ADD       | Add a new task");
    println!("LIST      | List all tasks");
    println!("EDIT      | Edit a task");
    println!("EDIT 1    | Edit task number 1 (replace 1 with the task number)");
    println!("DONE 1    | Mark task number 1 as done (replace 1 with the task number)");
    println!("SEARCH    | Search tasks by tag or name (replace tag with the tag or name)");
    println!("EXIT      | Exit the program");
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
        "SEARCH" => (5, 0),
        "EXIT" => (6, 0),
        _ => (0, 0),
    }
}



fn list_tasks(tasks: &Vec<Task>) {
    println!("\nTasks:");
    for (i, task) in tasks.iter().enumerate() {
        print_task(task, i)
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

fn search(tasks: &Vec<Task>) {
    println!("Enter tag or name to search:");
    let mut filter = String::new();
    io::stdin().read_line(&mut filter).expect("Failed to read line");
    let filter = filter.trim();
    println!("\nTasks:");
    for (i, task) in tasks.iter().enumerate() {
        if task.name.contains(filter) || task.tags.contains(&filter.to_string()) {
            print_task(task, i);
        }
    }
}

fn print_task(task: &Task, index: usize) {
    if task.done {
        println!("{}. [X] {} - Tags: {:?}", index + 1, task.name, task.tags);
    } else {
        println!("{}. [ ] {} - Tags: {:?}", index + 1, task.name, task.tags);
    }
}