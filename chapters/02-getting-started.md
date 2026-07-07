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

