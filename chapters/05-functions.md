# Chapter 5: Designing Functions in Rust

Functions are the fundamental building blocks of a Rust codebase. In this chapter, you will learn how to design, write, test, and declare modular functions. We will explore parameter conventions, returning values, the difference between statements and expressions, and advanced parameter adapters like `AsRef` and `Into`.

---

## 1. Syntax of a Rust Function

We declare functions in Rust using the `fn` keyword. Rust uses **snake_case** as the standard style for function and variable names, where all letters are lowercase and words are separated by underscores.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Let us dissect this declaration:
1. **`fn`**: The keyword introducing a function.
2. **Parameters (`a: i32, b: i32`)**: Every parameter must have an explicit type annotation. Unlike some dynamic languages, the compiler does not infer types for function signatures, ensuring your APIs are stable and easy to read.
3. **Return Type (`-> i32`)**: The return type is defined with an arrow `->`. If a function does not return a value, the return signature is omitted, which implicitly returns the empty tuple, also known as the **Unit Type**: `()`.

---

## 2. Statements vs. Expressions

Rust is an **expression-oriented** language. This represents a major difference from statement-heavy languages like C, Java, or Python.

- **Statements**: Instructions that perform some action but **do not return a value**. In Rust, lines ending with a semicolon `;` are usually statements.
  ```rust
  let x = 6; // This is a statement. It does not return any value.
  ```
- **Expressions**: Evaluate to a resulting value. Most code blocks we write in Rust evaluate to an expression.
  ```rust
  let y = {
      let x = 3;
      x + 1 // This is an expression! Note that it does NOT end with a semicolon!
  }; // y binds to 4
  ```

In our `add(a, b)` function:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // No semicolon! This is the expression evaluated and returned.
}
```
If we put a semicolon at the end of `a + b;`, it would turn the expression into a statement, returning the unit type `()` instead of our expected integer, causing a compiler error.

---

## 3. Parameter Passing: References vs. Ownership

When designing a function API, choose how parameters are passed based on your ownership needs:

1. **Pass by Reference (`&T`)**: Let the function read or inspect data without taking ownership. This is the most common parameter type in Rust.
2. **Pass by Mutable Reference (`&mut T`)**: Let the function mutate callers' data without taking ownership.
3. **Pass by Value (`T`)**: Transfer ownership of the resource to the function. Use this if the function needs to store, consume, or transform the resource.

```rust
fn inspect_string(s: &String) {
    println!("Inspecting: {}", s);
}

fn edit_string(s: &mut String) {
    s.push_str(" appended");
}

fn discard_string(s: String) {
    println!("Destroying ownership: {}", s);
} // s is dropped here!
```

---

## 4. Advanced Parameter Ergonomics: AsRef and Into

As you write more Rust, you will notice that converting between reference types (like `&String` to `&str`) or owned types (like `&str` to `String`) can be verbose. Rust provides two standard traits to clean up your function designs: `AsRef` and `Into`.

### A. AsRef<str> for Free String Slices
The `AsRef` trait allows a function to accept any argument that can be easily converted into a reference of another type. This is incredibly useful for string inputs; it lets callers pass in literal `&str` reference variables, owned `String` models, or even standard `&String` values without manual conversions:

```rust
fn print_label<S: AsRef<str>>(label: S) {
    let slice: &str = label.as_ref();
    println!("{}", slice);
}

fn main() {
    print_label("static slice"); // Works!
    print_label(String::from("owned string")); // Works!
}
```

### B. Into<String> for Flexible Owned Inputs
The `Into` trait lets your function accept any type that can be converted into an owned type. This shifts the performance cost of cloning or creating strings from the function implementation to the caller:

```rust
struct User {
    username: String,
}

impl User {
    // Caller can pass &str (will be allocated) or String (will be moved)
    fn new<S: Into<String>>(name: S) -> Self {
        User {
            username: name.into(),
        }
    }
}
```

---

## Exercises

### Exercise 1: Modular Join Names
1. Open the exercise file at [rust-learn-guide/examples/join_names/src/lib.rs](rust-learn-guide/examples/join_names/src/lib.rs).
2. Implement the following function:
   ```rust
   pub fn join_names<A: AsRef<str>>(a: A, b: A) -> String {
       format!("{} {}", a.as_ref(), b.as_ref())
   }
   ```
3. Test your logic by executing tests for the package:
   ```bash
   cargo test -p join_names
   ```

### Exercise 2: Double a Value
1. Write a function `double_value` that takes an `i32` and returns its double.
2. Verify that there is no semicolon after the expression.
3. Call it from your test framework. Solutions are available in [rust-learn-guide/solutions/05-functions.md](rust-learn-guide/solutions/05-functions.md).

Now that we have variables, ownership, borrowing, and functions in place, let us reflect on our progress and set ourselves up to learn custom compound types. Head over to [rust-learn-guide/chapters/06-next-steps.md](rust-learn-guide/chapters/06-next-steps.md).
