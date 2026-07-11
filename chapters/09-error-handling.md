# Chapter 9: Robust Error Handling in Rust

Errors are an inevitable reality of software development. Unlike many modern languages that use exceptions (such as Java, C#, or Python), Rust does not support runtime exceptions. Instead, it categorizes errors into two distinct archetypes: **Unrecoverable Errors** (using the `panic!` macro) and **Recoverable Errors** (using the `Result` enum). 

By making errors types part of function signatures, Rust makes failures explicit, robust, and highly performant.

---

## 1. Unrecoverable Errors with panic!

Sometimes, things go so wrong that your program simply cannot continue safely (e.g., trying to access an out-of-bounds array index, or encountering a fatal system configuration state). For these scenarios, Rust provides the `panic!` macro.

When a panic occurs:
1. The program prints a failure message.
2. The runtime cleans up and unwinds the stack, deallocating resources.
3. The program exits.

```rust
fn main() {
    println!("Starting application");
    panic!("Critical error: Database is unreachable!");
    // println!("This line is unreachable and will not compile or run");
}
```

*When to panic*: Use `panic!` only for bugs or unrecoverable situation states (like index out-of-bounds, contract violations, or during testing checks). For expected failures (like file not found or parsing input failure), use recoverable errors.

---

## 2. Recoverable Errors with Option and Result

For scenario flows where errors are expected and should be handled gracefully by your code, Rust uses two enum types.

### A. The Option Type (Missing Values)
The standard library `Option<T>` is used when a function returns a value that might be absent or null:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example of use:
```rust
fn find_username(user_id: u32) -> Option<String> {
    if user_id == 42 {
        Some(String::from("rust_connoisseur"))
    } else {
        None
    }
}
```

To access the value safely inside an `Option`, developers must handle the `None` case using `match`, `if let`, or methods like `unwrap_or`:

```rust
let username_opt = find_username(9);

// 1. Using unwrap_or to fallback to a default
let name = username_opt.unwrap_or(String::from("Guest"));

// 2. Using match
match find_username(42) {
    Some(username) => println!("User found: {}", username),
    None => println!("User not found"),
}
```

---

### B. The Result Type (Fallible Operations)
The `Result<T, E>` enum is used for operations that can either succeed (`Ok(T)`) or fail with a specific error type (`Err(E)`):

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Let us look at a standard example: parsing a string into an integer.

```rust
fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}

fn main() {
    let result = parse_number("abc");
    
    match result {
        Ok(num) => println!("Parsed integer successfully: {}", num),
        Err(e) => println!("Failed to parse integer: {}!", e),
    }
}
```

The compiler guarantees you check the `Err` branch. Your code will not compile if you try to use the parsed integer without unwrapping or matching the `Result` first!

---

## 3. Propagating Errors with the ? Operator

If you write a function that calls other fallible functions, you ofttimes want to pass (or **propagate**) any errors back up to the caller to handle.

In older versions of Rust, this required verbose matching blocks:
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let mut file_result = File::open("username.txt");
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // Explicitly return early
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

Rust simplifies this drastically with the **`?` operator**:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // Note the query mark!
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

### How the `?` Operator Works:
1. If the value evaluates to `Ok(value)`, the expression yields the wrapped `value`, and execution continues.
2. If the value evaluates to `Err(e)` or `None`, the `?` operator immediately returns the error from the surrounding function.
3. *Note*: The `?` operator can only be used in functions that return a type compatible with the value the `?` is called on (such as returning `Result` or `Option`).

---

## Exercises

### Exercise 1: Math Safe Division
1. Browse solution guidelines inside [rust-learn-guide/solutions/09-error-handling.md](rust-learn-guide/solutions/09-error-handling.md).
2. Implement a function:
   ```rust
   pub fn divide(a: f64, b: f64) -> Option<f64> {
       if b == 0.0 {
           None
       } else {
           Some(a / b)
       }
   }
   ```
3. Test how it handles division by zero and check that it returns `None`.

### Exercise 2: Chaining Parse Actions
1. Implement a helper function `parse_and_double(input: &str) -> Result<i32, std::num::ParseIntError>` that parses a string to `i32` using `?`, doubles it, and returns the finished product inside `Ok(...)`.
2. Write a test passing `"15"` and verify it returns `Ok(30)`.

Now that we can write comprehensive, robust functions and structures with safe error boundaries, let us organize our code across files using modules. Head over to [rust-learn-guide/chapters/10-modules-and-crates.md](rust-learn-guide/chapters/10-modules-and-crates.md).
