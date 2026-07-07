# Introduction

Welcome to the Rust Learning Guide — a hands-on, chapter-by-chapter path to mastering practical Rust.

Learning goals
- Understand Rust's goals and where it is commonly used
- Learn tooling: `rustup`, `cargo`, `rustc`, and `cargo test`
- Build and run small programs; write simple unit tests

How to use this guide
- Read the short chapter text, then complete the exercises at the end.
- Examples live in the `examples/` folder and are runnable with `cargo run -p <example>`.
- Solutions are in `solutions/`; attempt exercises before reading them.

Solutions index
- A curated index of solutions is available at `solutions/README.md` — use it to check your answers after attempting exercises.

Prerequisites
- A computer with network access
- Basic command-line familiarity (opening a terminal, running commands)

Roadmap
- Chapters 1–2: setup, first program, and basic tooling
- Chapters 3–6: ownership, borrowing, lifetimes, structs, enums
- Later chapters: async, testing, package organization, capstone project

Next: follow the steps in Chapter 2 to install Rust and create your first project.

Quick check
- Q: What command creates a new Rust package? A: `cargo new <name>`

What you'll learn in Chapter 1

- Rust's philosophy: ownership, borrowing, and safety-by-default.
- How to read simple Rust code and run an example with `cargo run`.
- How to write a tiny program, add a unit test, and run `cargo test`.

Exercises

1. Run the included `hello_world` example: change the greeting message, run it with `cargo run -p hello_world`, and add a unit test that asserts the greeting text.
2. Read the `greet_name` example and modify it to accept a surname as a second argument. Add a test covering the new behavior.

Learning tips

- Start small: change one function at a time and run the tests frequently.
- Prefer compiler errors as guidance - they are precise and help you learn ownership and lifetimes.
- Use `cargo test -- --nocapture` to see test output while debugging.

Next steps

- After finishing these exercises, continue to Chapter 2 where we build a small CLI and introduce `clap`, `serde`, and basic persistence.
- If you get stuck, consult the `solutions/` folder for worked examples and hints.
