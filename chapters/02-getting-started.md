# Chapter 2: Getting Started with Rust

To write and compile Rust code, you need a local development environment. This chapter guides you through installing the official Rust toolchain via `rustup`, verifying your setup, and creating your very first Cargo project.

## Goals
- Install Rust and the standard compilation tools.
- Understand the core tools: `rustc`, `cargo`, and `rustup`.
- Create, run, and test a brand new compiled candidate.
- Understand Cargo's directory layout and file structures.

---

## Step 1: Install Rust with rustup

The recommended way to install Rust is using `rustup`, which manages Rust compiler versions and standard libraries across different operating systems.

### installation on Linux or macOS
Open your terminal and run the following command to download and execute the installer script:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation on Windows
If you are on Windows:
1. Navigate to https://rustup.rs in your web browser.
2. Download the installer executable `rustup-init.exe`.
3. Run the installer and follow the on-screen instructions. 
   *(Note: You will also need the C++ Build Tools for Visual Studio, which the installer will prompt you to download if they are missing.)*

After the installation completes, restart your terminal or run this command to configure your shell:
```bash
source $HOME/.cargo/env
```

---

## Step 2: Verify Your Installation

Let us check that the Rust compiler and its container package manager are installed correctly. Run the following command:

```bash
rustc --version
```
This is the compiler. Output should look similar to:
`rustc 1.78.0 (9b00956e5 2024-05-03)`

Now, check Cargo, Rust's build system and package manager:
```bash
cargo --version
```
Output:
`cargo 1.78.0 (9b00956e5 2024-05-03)`

Finally, check your active toolchains:
```bash
rustup show
```
This displays your default compiler target, host architecture, and other components currently loaded (such as `rust-std` or `rust-analyzer`).

---

## Step 3: Understanding Cargo and the Project Layout

Cargo is Rust's orchestrator. It acts as a:
1. **Package Manager**: Fetches, builds, and updates your library dependencies.
2. **Build System**: Compiles your modules and links binaries.
3. **Test Runner**: Discovers and parallelizes unit and integration tests.

Let us create a new project called `hello_rust` from scratch:

```bash
cargo new hello_rust
cd hello_rust
```

Let us look at the structure that Cargo has generated for you:
```text
hello_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

### 1. Cargo.toml
This file represents your project's manifest in TOML (Tom's Obvious, Minimal Language). It contains settings, metadata, and dependencies:

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# Your external library dependencies will go here
```

### 2. src/main.rs
This is the main entry point of your executable binary. Open this file and look at the default template logic generated:

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## Step 4: Compiling and Running Your Project

Cargo makes it extremely clean to build and run your projects.

### 1. Build without running
To compile your application and prepare an executable without executing it immediately, run:
```bash
cargo build
```
This compilation creates a debug build in `target/debug/hello_rust` (or `hello_rust.exe` on Windows). Debug builds compile quickly and maintain diagnostic symbols, but run slower because optimizations are disabled.

### 2. Build for production (Optimized)
When you are ready to distribute your application, compile it with high speed optimizations enabled:
```bash
cargo build --release
```
This builds the binary in `target/release/hello_rust`. It takes longer to compile but runs containing massive hardware optimizations.

### 3. Run the application in one step
Usually, you want to compile and immediately execute the output. Run:
```bash
cargo run
```
Cargo checks if any of your files have changed, incrementally recompiles them, and runs the final binary:
```text
   Compiling hello_rust v0.1.0 (D:\Projects\hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello_rust`
Hello, world!
```

---

## Exercises

### Exercise 1: Customize Your Greeting
1. Open your main file at [rust-learn-guide/examples/hello_world/src/main.rs](rust-learn-guide/examples/hello_world/src/main.rs).
2. Change the text to print a custom greeting (e.g., `Hello, Rust Learner!`).
3. Run `cargo run -p hello_world` from the `rust-learn-guide/examples` workspace directory to execute the hello_world package and check your changes.

### Exercise 2: Create a Greet Name Binary
1. Open the file [rust-learn-guide/examples/greet_name/src/main.rs](rust-learn-guide/examples/greet_name/src/main.rs) and look at the source code.
2. Edit its logic to print your own name.
3. Test your configuration using:
   ```bash
   cargo test -p greet_name
   ```

Now that you have your environment set up and can compile basic programs, let us dive into the core syntax of the language in [rust-learn-guide/chapters/03-variables-and-types.md](rust-learn-guide/chapters/03-variables-and-types.md).
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

