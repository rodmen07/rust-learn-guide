# Rust Learn Guide

A hands-on guide to learning Rust. This repository contains short chapters, exercises, and runnable examples.

## Structure
- `chapters/` - guide chapters
- `examples/` - small Cargo projects that demonstrate concepts
- `examples/todo_cli/` - a capstone-style CLI example with tests
- `solutions/` - chapter-aligned exercise solutions

## Start here

1. Install Rust with `rustup`
2. Open the first chapter in `chapters/01-introduction.md`
3. Run the examples:

```bash
cd examples/hello_world
cargo run

cd ../todo_cli
cargo test
cargo run -- "Write more Rust"
cargo run -- done 1
```

## Progression

- Learn the language basics in `chapters/`
- Practice with the `hello_world` and `todo_cli` examples
- Build your own capstone project after the final chapter

## Solutions

Exercise solutions live in `solutions/` and follow the chapter numbering. Start with `solutions/README.md` for the index.

## Landing Page

Start with the chapter list in `SUMMARY.md`, then follow the examples in order.
