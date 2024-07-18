use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

const TODO_FILE: &str = "todo.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: Missing task description.");
            } else {
                let task = args[2..].join(" ");
                add_task(&task);
            }
        }
        "list" => {
            list_tasks();
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Error: Missing task number.");
            } else {
                let task_number: usize = args[2].parse().unwrap_or(0);
                remove_task(task_number);
            }
        }
        _ => {
            print_help();
        }
    }
}

fn print_help() {
    println!("Usage:");
    println!("  todo add <task>     - Add a new task");
    println!("  todo list           - List all tasks");
    println!("  todo remove <num>   - Remove a task by number");
}

fn add_task(task: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(TODO_FILE)
        .expect("Unable to open file");
    writeln!(file, "{}", task).expect("Unable to write to file");
    println!("Added task: {}", task);
}

fn list_tasks() {
    let file = OpenOptions::new().read(true).open(TODO_FILE);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                let line = line.expect("Unable to read line");
                println!("{}: {}", index + 1, line);
            }
        }
        Err(_) => {
            println!("No tasks found.");
        }
    }
}

fn remove_task(task_number: usize) {
    let file = OpenOptions::new().read(true).open(TODO_FILE);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let tasks: Vec<String> = reader.lines().filter_map(Result::ok).collect();
            if task_number == 0 || task_number > tasks.len() {
                eprintln!("Error: Invalid task number.");
                return;
            }
            let new_tasks: Vec<String> = tasks
                .into_iter()
                .enumerate()
                .filter_map(|(index, task)| {
                    if index + 1 == task_number {
                        None
                    } else {
                        Some(task)
                    }
                })
                .collect();
            fs::write(TODO_FILE, new_tasks.join("\n")).expect("Unable to write to file");
            println!("Removed task number {}", task_number);
        }
        Err(_) => {
            eprintln!("Error: Unable to open file.");
        }
    }
}
