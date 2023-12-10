use std::env;
mod todo;
use todo::Todo;

const STORAGE_FILE: &str = "todo.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        show_usage();
        return;
    }

    let mut todo = Todo::new(STORAGE_FILE.to_string());

    let command = args[1].clone();

    if args.len() == 2 {
        match command.as_str() {
            "clean" => todo.clean_tasks(),
            "show" => todo.show_tasks(),
            _ => show_usage(),

        }
    }

    else if args.len() == 3  {
        match command.as_str() {
            "add" => todo.add_task(args[2].clone()),
            "complete" => todo.complete_task(args[2].parse::<usize>().unwrap()),
            "delete" => todo.delete_task(args[2].parse::<usize>().unwrap()),
            _ => show_usage(),
        }
    }

    else {
        show_usage();
    }
}

fn show_usage() {
    println!("ToDo List usage: ");
    println!("init     -> Initiates the storage");
    println!("add      -> Add a new task");
    println!("show     -> Show all tasks");
    println!("complete -> Complete a task");
    println!("delete   -> Delete a task");
    println!("clean    -> Deletes all tasks");
}

