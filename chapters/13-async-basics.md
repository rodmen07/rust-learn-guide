# Chapter 13: Core Asynchronous Programming in Rust

When building network services, databases, or interactive desktop applications, threads spend a lot of time waiting on I/O (like waiting for a web request to finish or a file to load). Classic synchronous threads block on these operations, idling system resources. 

To solve this, Rust provides a powerful model for asynchronous execution, enabling single threads to execute millions of tasks concurrently.

This chapter covers the foundation of Rust's **async/await** architecture, the lazy **Future** model, and the role of third-party **Async Runtimes** like Tokio.

---

## 1. What is Asynchronous Programming?

In synchronous code, execution blocks until each line completes:
```rust
let response = fetch_from_api(); // blocks thread!
println!("Got response!"); // awaits completion
```

In asynchronous code, we yield resource blocks when waiting on I/O, allowing our processor execution flow to make progress on other concurrent tasks:
```rust
let response_future = fetch_from_api(); // returns a future immediately
```

---

## 2. Futures in Rust

At the heart of Rust's async system is the `Future` trait. A `Future` represents an asynchronous operation that may or may not have completed yet.

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Item>;
}
```

### Key Architectural Fact:
In many languages (like Node.js, C#, or Go), async tasks are managed by an active runtime scheduler inside a background pool immediately upon creation. 

In Rust, **Futures are completely lazy**. They do nothing and make no progress until they are explicitly polled. They compile down to state machines, meaning there is zero allocation or garbage collection overhead to track futures!

---

## 3. The async and await Keywords

We write asynchronous code using the keywords `async` and `await`:

- **`async`**: Converts a block of code or function into a state machine that implements `Future`.
- **`.await`**: Pauses the current function execution, handing control back to the async executor to work on other items until the awaited future is ready to yield its output.

```rust
// async fn returns a Future wrapping a String
async fn fetch_user_data() -> String {
    // some network operations
    "User Data".to_string()
}

async fn run_processes() {
    let future = fetch_user_data(); // no work is executed yet!
    
    let result = future.await; // pauses execution, executes block, yields result
    println!("{}", result);
}
```

---

## 4. The Need for an Async Runtime

Because Rust's standard library does not incorporate a built-in asynchronous executor or scheduler, you cannot execute async mains or wait on futures out-of-the-box. Instead, we use community standard library runtimes:

- **Tokio**: The industry standard for heavy network servers, APIs, and complex async multi-tasking.
- **async-std**: A lightweight alternative mimicking the standard library API surface.

To use Tokio, you declare it as a dependency in your package manifest and annotate your `main` entry point:

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

```rust
// src/main.rs
#[tokio::main] // Macro configuring Tokio's runtime loop
async fn main() {
    println!("Starting Tokio loop");
    let msg = fetch_user_data().await;
    println!("{}", msg);
}
```

---

## Exercises

### Exercise 1: Exploring Async Syntax
1. Inspect solutions for async patterns at [rust-learn-guide/solutions/13-async-basics.md](rust-learn-guide/solutions/13-async-basics.md).
2. Write a mock asynchronous function:
   ```rust
   async fn fetch_data() -> String {
       "done".to_string()
   }
   ```
3. Read the code and explain how its lazy future behavior is polled.

### Exercise 2: Defining a mock file-loader
Create an async function `load_config()` that returns `Result<String, ()>`. Use unit tests inside your async environment to wait on the configuration outcome safely.

Let us now apply everything we've read to build a beautiful CLI candidate! Proceed to the Capstone Project in [rust-learn-guide/chapters/14-capstone-project.md](rust-learn-guide/chapters/14-capstone-project.md).
