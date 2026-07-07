use structs_enums::Task;
use std::env;
use std::fs;
use std::path::PathBuf;

fn tasks_file() -> PathBuf {
    PathBuf::from("tasks.json")
}

fn load_tasks(path: &PathBuf) -> Vec<Task> {
    if path.exists() {
        let s = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&s).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_tasks(path: &PathBuf, tasks: &Vec<Task>) {
    if let Ok(s) = serde_json::to_string_pretty(tasks) {
        let _ = fs::write(path, s);
    }
}

fn print_tasks(tasks: &Vec<Task>) {
    for (i, t) in tasks.iter().enumerate() {
        println!("{}: [{}] {}", i + 1, if t.done { 'x' } else { ' ' }, t.title);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let file = tasks_file();
    let mut tasks: Vec<Task> = load_tasks(&file);

    if args.is_empty() {
        eprintln!("usage: add <title> | remove <index> | list");
        return;
    }

    match args[0].as_str() {
        "add" => {
            let title = args[1..].join(" ");
            let task = Task::new(title);
            tasks.push(task);
            save_tasks(&file, &tasks);
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
                save_tasks(&file, &tasks);
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
