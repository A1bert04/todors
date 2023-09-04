use std::io;
use crate::Task;

pub fn edit_task(task: usize, tasks: &mut Vec<Task>) {
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

pub fn mark_task_done(index: usize, tasks: &mut Vec<Task>) {
    if let Some(task) = tasks.get_mut(index - 1) {
        task.done = true;
        println!("Task number {} marked as done", index);
    } else {
        println!("Invalid index");
    }
}