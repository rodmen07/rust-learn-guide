# Error Handling

Rust uses `Result` and `Option` to make failure explicit.

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
```

Use `match` or `?` to handle errors cleanly.

`Result` works well when something can succeed or fail.

```rust
fn parse_count(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}
```

You can bubble errors up with `?`.

```rust
fn double_parsed(input: &str) -> Result<i32, std::num::ParseIntError> {
    let value = input.parse::<i32>()?;
    Ok(value * 2)
}
```

## Exercise

1. Write a function that returns `Option<i32>`.
2. Return `None` for invalid input.
3. Handle the result with `match`.

## Challenge

1. Write a function that returns `Result<i32, _>`.
2. Use `?` in a helper function.
3. Print a friendly error message.
