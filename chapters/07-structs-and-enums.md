# Chapter 7: Expressing Custom Types with Structs and Enums

So far, we have worked exclusively with Rust's primitive data types (integers, strings, floats, tuples, and arrays). To build real applications, you must construct more complex, custom types that mirror your domain. In Rust, you mold custom types using two main structures: **Structs** and **Enums**.

---

## 1. Structs: Packaging Related Data

A struct (short for structure) is a custom data type that lets you package together multiple related values under a single named unit.

Rust supports three types of structs: **Classic Structs**, **Tuple Structs**, and **Unit-Like Structs**.

### A. Classic Structs (Named Fields)
Classic structs are the most common. Each field inside the struct has an explicit name and type:

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
```

To create an instance of this struct, define values for each field:
```rust
let mut user1 = User {
    username: String::from("alice_dev"),
    email: String::from("alice@example.com"),
    active: true,
    sign_in_count: 1,
};

// Accessing and modifying fields
user1.email = String::from("alice_updated@example.com");
```
*(Note: If the instance is mutable, all of its fields are mutable. Rust does not support field-level mutability annotations.)*

### Field Init Shorthand
If you have variables with the exact same name as your struct fields, you can use the initialization shorthand:
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email, // Shorthand for email: email
        username, // Shorthand for username: username
        active: true,
        sign_in_count: 1,
    }
}
```

### B. Tuple Structs
Tuple structs are useful when you want to give a tuple a distinct type name, but naming each field is redundant:

```rust
struct Color(i32, i32, i32);
struct Point(f64, f64, f64);

let black = Color(0, 0, 0);
let origin = Point(0.0, 0.0, 0.0);
```
Even though `Color` and `Point` contain identical field types (`i32` or `f64`), they represent entirely separate types. You cannot pass a `Color` where a `Point` is expected, preserving strict safety.

### C. Unit-Like Structs
Unit-like structs have no fields at all. They are useful when you need to implement a trait on a type but do not have any state to store:

```rust
struct AlwaysEqual;
```

---

## 2. Implementing Methods and Associated Functions

In Rust, we place behavior and structural data together, but declare them separately. Data is defined in the `struct` block, while behaviors (methods and functions) are defined inside an `impl` (implementation) block:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 1. Associated Function (Constructor)
    // Does not take self; called using Rectangle::square(10)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // 2. Read-Only Method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 3. Mutable Method
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}
```

### Understanding `self` parameters:
- **`&self`**: Borrows the data immutably. Used to read the struct's properties.
- **`&mut self`**: Borrows the data mutably. Used to update the struct's properties.
- **`self`**: Takes full ownership of the struct. This consumes the instance (making it unavailable to the caller afterward), which is useful for conversion methods like `into_string()`.

---

## 3. Enums: Modeling Alternatives and Rich States

Enums (enumerations) let you declare a type that can be one of several possible variants. Where a struct groups fields together, an enum lets you choose only **one** variant.

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

### Enums with Data
One of Rust's most brilliant features is that enums can store data inside each variant. This lets you construct clean and expressive state models:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8), // Tuple-like variant
    V6(String),         // Tuple-like variant
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::to_string("::1"));
```

You can even add complex struct-like fields to enum variants:
```rust
enum Message {
    Quit, // No data
    Move { x: i32, y: i32 }, // Anonymous struct
    Write(String), // Single String
}
```

---

## 4. Modeling Domain State with Option and Result

Instead of having unsafe null pointers or loose integer error codes, Rust uses the standard library enums `Option` and `Result` to govern fallible scenarios.

### The Option Enum
```rust
enum Option<T> {
    Some(T),
    None,
}
```
If a value might be missing, model it as `Option<T>`.

### The Result Enum
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
If an operation can succeed or fail with an error, model it as `Result<T, E>`.

---

## Exercises

### Exercise 1: Task and Command Modeling
1. Open the runnable example cargo project in [rust-learn-guide/examples/structs_enums/src/lib.rs](rust-learn-guide/examples/structs_enums/src/lib.rs).
2. Implement a `Task` struct and an associated implementation block that lets you instantiate new tasks and mark them as done.
3. Test your behavior by running your test pipeline:
   ```bash
   cargo test -p structs_enums
   ```

### Exercise 2: Defining an associated constructor
1. Open [rust-learn-guide/examples/structs_enums/src/main.rs](rust-learn-guide/examples/structs_enums/src/main.rs).
2. Define an associated constructor function `new` for `Task` that returns `Self` with `done` initialized to false. Check [rust-learn-guide/solutions/07-structs-and-enums.md](rust-learn-guide/solutions/07-structs-and-enums.md) for matching reference patterns.

Now that we have rich data types, we need a clean, structured way to extract information from them depending on their variants. Let us proceed to pattern matching in [rust-learn-guide/chapters/08-pattern-matching.md](rust-learn-guide/chapters/08-pattern-matching.md).
