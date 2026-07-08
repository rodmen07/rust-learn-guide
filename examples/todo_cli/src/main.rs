use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use todo_cli::{format_tasks, parse_tasks, serialize_tasks, Task};

const TASKS_FILE: &str = "tasks.json";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,

    /// Add a task by providing the title directly
    title: Option<String>,
    /// Path to tasks file (defaults to tasks.txt)
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the current task list without changing anything
    List,

    /// Mark a task done by its 1-based index
    Done { index: usize },
    /// Remove a task by its 1-based index
    Remove { index: usize },
    /// Edit a task's title by its 1-based index
    Edit { index: usize, title: Vec<String> },
}

fn main() {
    let cli = Cli::parse();
    let tasks_file = cli.file.clone().unwrap_or_else(|| PathBuf::from(TASKS_FILE));
    let mut tasks = load_tasks_from(&tasks_file).unwrap_or_else(|_| default_tasks());

    match (&cli.command, &cli.title) {
        (Some(Commands::List), _) => {}
        (Some(Commands::Done { index }), _) => {
            if let Some(task) = tasks.get_mut(index.saturating_sub(1)) {
                task.done = true;
                if let Err(error) = save_tasks_to(&tasks, &tasks_file) {
                    eprintln!("warning: could not save tasks: {}", error);
                }
            } else {
                eprintln!("warning: task {} does not exist", index);
            }
        }
        (Some(Commands::Remove { index }), _) => {
            if *index == 0 || *index > tasks.len() {
                eprintln!("warning: task {} does not exist", index);
            } else {
                tasks.remove(index.saturating_sub(1));
                if let Err(error) = save_tasks_to(&tasks, &tasks_file) {
                    eprintln!("warning: could not save tasks: {}", error);
                }
            }
        }
        (Some(Commands::Edit { index, title }), _) => {
            if *index == 0 || *index > tasks.len() {
                eprintln!("warning: task {} does not exist", index);
            } else {
                let new_title = title.join(" ");
                if let Some(task) = tasks.get_mut(index.saturating_sub(1)) {
                    task.title = new_title;
                    if let Err(error) = save_tasks_to(&tasks, &tasks_file) {
                        eprintln!("warning: could not save tasks: {}", error);
                    }
                }
            }
        }
        (None, Some(title)) => {
            tasks.push(Task {
                title: title.to_string(),
                done: false,
            });

            if let Err(error) = save_tasks_to(&tasks, &tasks_file) {
                eprintln!("warning: could not save tasks: {}", error);
            }
        }
        _ => {}
    }

    for line in format_tasks(&tasks) {
        println!("{}", line);
    }
}

fn default_tasks() -> Vec<Task> {
    vec![
        Task {
            title: String::from("Read the Rust book"),
            done: false,
        },
        Task {
            title: String::from("Build a small CLI"),
            done: true,
        },
    ]
}

fn load_tasks_from(path: &PathBuf) -> Result<Vec<Task>, std::io::Error> {
    if !path.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "tasks file missing"));
    }

    let content = fs::read_to_string(path)?;
    Ok(parse_tasks(&content))
}

fn save_tasks_to(tasks: &[Task], path: &PathBuf) -> Result<(), std::io::Error> {
    fs::write(path, serialize_tasks(tasks))
}
