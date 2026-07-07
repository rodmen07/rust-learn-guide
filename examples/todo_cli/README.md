# todo_cli

A tiny learning capstone CLI for the Rust guide.

Usage examples:

- Add a task (positional title):

```sh
cargo run -p todo_cli -- "Write docs"
```

- Mark task 1 done:

```sh
cargo run -p todo_cli -- done 1
```

Notes:
- Tasks are stored in `tasks.txt` in the current working directory.
- This example uses `clap` for simple argument parsing.
 - Default tasks file is now `tasks.json` (use `-f/--file` to override).
