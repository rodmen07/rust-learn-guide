# Testing

Rust includes built-in support for testing.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

Tests help you catch regressions early.

Use tests for small pure functions first, then move up to larger behavior.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

## Exercise

1. Write a small function.
2. Add a test for it.
3. Run `cargo test` in a project.

## Challenge

1. Add two more tests.
2. Test an error case.
3. Refactor the function without breaking the tests.
