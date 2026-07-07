# Async Basics

Async code helps you work with tasks that wait on I/O.

```rust
async fn fetch_data() -> String {
    "done".to_string()
}
```

In Rust, async functions return a future that must be executed by an async runtime.

Async is useful for network calls, file work, and other waiting tasks.

```rust
async fn load_message() -> String {
    "hello from async".to_string()
}
```

You usually run async code with a runtime like Tokio or async-std.

## Exercise

1. Read an async function example.
2. Learn what a future is.
3. Compare async code with regular synchronous code.

## Challenge

1. Find one async runtime.
2. Write a second async function.
3. Describe one place async would help in a project.
