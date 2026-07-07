# Solution: greet_name

Create a small function `greeting(name: &str) -> String` and a binary that prints the result.

Example implementation (in `examples/greet_name/src/main.rs`):

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", greeting("Rust learner"));
}
```

Unit test:

```rust
#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn returns_name() {
        assert_eq!(greeting("Alice"), "Hello, Alice!");
    }
}
```

Run locally:

```bash
cargo run -p greet_name
cargo test -p greet_name
```

Notes:
- This solution is intentionally compact so learners can focus on function design and tests.
- Try extending it by reading a name from `stdin` or adding CLI args.
