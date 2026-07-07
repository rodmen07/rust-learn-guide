# Getting Started

Install Rust with `rustup`, then check your setup:

```bash
rustc --version
cargo --version
```

Goals
- Install Rust and toolchain components
- Create and run your first Cargo project
- Run tests and inspect build artifacts

Step 1 — Install Rust
- Follow https://rustup.rs and run the installer for your platform.

Step 2 — Verify installation

```bash
rustc --version
cargo --version
rustup show
```

Step 3 — Create your first project

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

Try it: edit `src/main.rs` and print a different greeting. Then run `cargo run` again.

Using examples from this guide
- We provide `examples/hello_world` you can run with:

```bash
cargo run -p hello_world
```

Running tests

```bash
cargo test -p hello_world
```

Exercises
- Create a new project `greet_name` that prints `Hello, <your name>!`.
- Add a unit test that checks the greeting function returns the expected string.

Tooling tips

- Update Rust toolchain and components regularly:

```powershell
rustup update
rustup component add clippy rustfmt
```

- Run the linter and formatter:

```powershell
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Running the larger examples (CLI and tests)

- To run the `todo_cli` example interactively:

```powershell
cargo run -p todo_cli -- list
```

- Integration tests for CLI binaries create and run the example binary. On Windows you may want to set a temporary `CARGO_TARGET_DIR` to avoid file-lock conflicts during repeated runs (see Troubleshooting below).

Recommended exercises (chapter 2)

1. Run `cargo run -p todo_cli -- --help` and read the usage output.
2. Add a new subcommand `edit <index> <new-title>` to `examples/todo_cli` (exercise only — implement the parsing part and tests later).
3. Inspect `examples/todo_cli/src/lib.rs` and write an additional unit test for `parse_tasks` that verifies invalid JSON returns an error.

Troubleshooting (Windows) - expanded

- If you see "Access is denied" when Cargo attempts to create or write under `target/`, prefer a writable temporary path. Example using `%TEMP%`:

```powershell
$td = Join-Path $env:TEMP "cargo-target-$(Get-Random)"
$env:CARGO_TARGET_DIR = $td
cargo test -p todo_cli
```

- If you still experience intermittent locking, ensure no editor or virus scanner is holding build artifacts, or try closing other programs that might access the binary under `target/debug/`.

Next

- Chapter 3 introduces ownership and borrowing with small exercises that modify functions, add tests, and refactor code to use `struct`s and `enum`s.

Short quiz
- Q1: Which command runs tests? (A: `cargo test`)
- Q2: Where do compiled artifacts go by default? (A: `target/`)

Troubleshooting (Windows)
- If builds fail with "access is denied" or "file is being used by another process", try using an isolated `CARGO_TARGET_DIR` for repeated test runs:

```powershell
$env:CARGO_TARGET_DIR='D:\Projects\rust-learn-guide\examples\target-temp'
cargo test -p greet_name
```

This avoids Windows locking issues on `target/` when running tests and binaries repeatedly.

