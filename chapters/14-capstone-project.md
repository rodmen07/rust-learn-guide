# Chapter 14: Capstone Project: Building a Production-Ready Todo CLI

Now that you have studied Rust's entire core feature set- ownership, safety, custom types, modules, error propagation, and collections- you are ready to combine them into a real, robust, production-style application.

For your capstone project, you will study and extend a file-backed **Command-Line Interface (Todo CLI)**. This contains practical software architecture practices, using error handling, custom struct models, file I/O operations, unit tests, and modular architecture.

---

## 1. Project Specifications

Your application will handle five core actions:
1. **Add**: Insert a new task to your task catalog list.
2. **List**: Print and display all currently logged tasks with status markers.
3. **Done**: Mark a specific task as complete by index.
4. **Remove**: Delete a task from the list.
5. **Persistence**: Load and save your task array to a custom file on disk.

---

## 2. Code Architecture Overview

A professional Rust CLI separates concerns by dividing the codebase into two distinct parts:
- **`src/lib.rs` (Library)**: Houses all core domain models, file persistence, testing setups, and business logic.
- **`src/main.rs` (Binary Entrance)**: Handles user argument parsing, calls the library functions, and outputs pretty console layouts.

### A. The Core Task Struct
First, model your domain data structures inside [rust-learn-guide/examples/todo_cli/src/lib.rs](rust-learn-guide/examples/todo_cli/src/lib.rs):

```rust
pub struct Task {
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new<S: Into<String>>(title: S) -> Self {
        Task {
            title: name.into(),
            done: false,
        }
    }
}
```

---

## 3. Developing Disk Persistence

To prevent losing data whenever you close your terminal, you must save your tasks to a plain-text file (e.g., `tasks.txt`) on your drive.

We use Rust's standard file operations:
```rust
use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

pub fn save_tasks(tasks: &[Task], filepath: &str) -> io::Result<()> {
    let mut file = File::create(filepath)?;
    for task in tasks {
        let status = if task.done { "1" } else { "0" };
        writeln!(file, "{}|{}", status, task.title)?;
    }
    Ok(())
}

pub fn load_tasks(filepath: &str) -> io::Result<Vec<Task>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut tasks = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            let done = parts[0] == "1";
            let title = parts[1].to_string();
            tasks.push(Task { title, done });
        }
    }
    Ok(tasks)
}
```

---

## 4. Building the main execution entry

Inside [rust-learn-guide/examples/todo_cli/src/main.rs](rust-learn-guide/examples/todo_cli/src/main.rs), we parse command-line arguments to resolve actions:

```rust
use std::env;
use todo_cli::{load_tasks, save_tasks, Task};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = "tasks.txt";

    if args.len() < 2 {
        println!("Usage: cargo run -- [add <title> | list | done <index>]");
        return;
    }

    let command = &args[1];
    let mut tasks = load_tasks(filepath).unwrap_or_else(|_| Vec::new());

    match command.as_str() {
        "list" => {
            for (idx, task) in tasks.iter().enumerate() {
                let status = if task.done { "[X]" } else { "[ ]" };
                println!("{} {}: {}", idx + 1, status, task.title);
            }
        }
        "add" if args.len() > 2 => {
            let title = args[2..].join(" ");
            tasks.push(Task::new(title));
            save_tasks(&tasks, filepath).expect("Failed to write to file");
            println!("Task logged successfully!");
        }
        _ => println!("Invalid argument or parameter combo!"),
    }
}
```

---

## Exercises

### Exercise 1: Run the Capstone Project
1. Run and review the complete solution code under [rust-learn-guide/examples/todo_cli](rust-learn-guide/examples/todo_cli).
2. Start the CLI to log a new task:
   ```bash
   cargo run -p todo_cli -- add "Finish reading Rust book"
   ```
3. List the logged elements:
   ```bash
   cargo run -p todo_cli -- list
   ```

### Exercise 2: Implementing the Remove Command
1. Open [rust-learn-guide/examples/todo_cli/src/main.rs](rust-learn-guide/examples/todo_cli/src/main.rs) and add support for a "remove" command.
2. It should accept an integer index, remove that element from the vector using `tasks.remove(idx)`, save the results, and print a confirmation message.
3. Write a test case verifying your new feature in [rust-learn-guide/examples/todo_cli/tests/persistence.rs](rust-learn-guide/examples/todo_cli/tests/persistence.rs).

---

## Closing Words: You Are Now a Rustacean!

By completing this book, you have conquered the learning curve of Rust's unique memory-safety mechanisms. You are now equipped to join open-source Rust crates, design lightning-fast microservices, build dependable embedded devices, or construct web utilities using Axum or Bevy!

Keep coding, keep compiling, and welcome to the vibrant community of **Rustaceans**! Check out review solutions in [rust-learn-guide/solutions/14-capstone-project.md](rust-learn-guide/solutions/14-capstone-project.md).
