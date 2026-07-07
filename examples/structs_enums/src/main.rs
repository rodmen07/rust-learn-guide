use structs_enums::{Task, Command};
use std::env;

fn print_tasks(tasks: &Vec<Task>) {
    for (i, t) in tasks.iter().enumerate() {
        println!("{}: [{}] {}", i + 1, if t.done { 'x' } else { ' ' }, t.title);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut tasks: Vec<Task> = Vec::new();

    if args.is_empty() {
        eprintln!("usage: add <title> | remove <index> | list");
        return;
    }

    match args[0].as_str() {
        "add" => {
            let title = args[1..].join(" ");
            let task = Task::new(title);
            tasks.push(task);
            print_tasks(&tasks);
        }
        "remove" => {
            if args.len() < 2 {
                eprintln!("remove requires an index");
                return;
            }
            if let Ok(i) = args[1].parse::<usize>() {
                if i == 0 || i > tasks.len() {
                    eprintln!("no such task");
                    return;
                }
                tasks.remove(i - 1);
                print_tasks(&tasks);
            } else {
                eprintln!("invalid index");
            }
        }
        "list" => {
            print_tasks(&tasks);
        }
        other => {
            eprintln!("unknown command: {}", other);
        }
    }
}
