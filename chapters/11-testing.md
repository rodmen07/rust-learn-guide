# Chapter 11: Testing in Rust

Writing tests is a fundamental part of the software development lifecycle. Rust features a built-in testing framework that is native to the language and managed directly by Cargo. You don't need any third-party libraries or external assertion runners to write high-quality tests in Rust.

This chapter explains how to write **Unit Tests**, create separate **Integration Tests**, write custom assertions, handle failures using `should_panic`, and execute your test suite with cargo commands.

---

## 1. How to Write a Unit Test

In Rust, **Unit Tests** live in the same source files as your implementation code. By convention, they are placed in a helper module named `tests` at the bottom of the file, decorated with the `#[cfg(test)]` block.

```rust
// In src/math.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 1. The tests module is conditionally compiled only when running 'cargo test'
#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    // 2. The #[test] attribute denotes a test case
    #[test]
    fn test_adds_numbers() {
        assert_eq!(add(2, 3), 5); // Assertion
    }
}
```

### The `#[cfg(test)]` Attribute:
- The `cfg` attribute stands for **configuration**. This tells Rust to *only* compile the marked module when we run `cargo test`.
- It completely strips the tests from your final production binaries, keeping your executable sizes small and fast.

---

## 2. Core Assertion Macros

Rust's standard library provides three primary macros to assert code behavior:

1. **`assert!(expression)`**: Evaluates a boolean expression. If it is false, the test panics and fails.
   ```rust
   assert!(true);
   ```
2. **`assert_eq!(left, right)`**: Asserts that two values are equal. If they are not, it outputs a detailed diagnostic printing both values.
   ```rust
   assert_eq!(2 + 2, 4);
   ```
3. **`assert_ne!(left, right)`**: Asserts that two values are not equal.
   ```rust
   assert_ne!(2 + 2, 5);
   ```

---

## 3. Testing for Panics using should_panic

Sometimes, you want to verify that your code panics under specific invalid parameters (such as trying to draw from an empty queue). We annotate these tests with `#[should_panic]`:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn test_value_greater_than_100() {
        Guess::new(150); // This should panic! The test passes because a panic was expected.
    }
}
```

By providing `expected = "..."`, we verify that the panic message contains that specific warning, ensuring our code did not panic for a different, unintended reason.

---

## 4. Integration Tests

While unit tests are for small, localized items, **Integration Tests** verify that multiple modules of your crate work together correctly. 
- Integration tests live in a dedicated `tests/` directory at the root of your project, next to your `src/` folder.
- Each file in the `tests/` directory is compiled as its own separate crate, interacting with your library solely through its public API.

Let us look at a typical integration test:
```rust
// In tests/integration_test.rs
use hello_rust::math; // Imports our public library math module

#[test]
fn test_integration_add() {
    assert_eq!(math::add(10, 20), 30);
}
```

---

## 5. Running Tests with Cargo

To run your entire test suite, simply run:
```bash
cargo test
```

### Handy Custom Test Commands:
1. **Run a single test**: Pass the name of a test function to run only that test:
   ```bash
   cargo test test_adds_numbers
   ```
2. **Filter and run multiple tests**: Pass any matching substring to run all tests with matching names:
   ```bash
   cargo test integration
   ```
3. **Show console stdout prints**: By default, Rust silences outputs from passing tests. To see `println!` outputs on successful tests, run:
   ```bash
   cargo test -- --show-output
   ```

---

## Exercises

### Exercise 1: Writing and Running Unit Tests
1. Open the source file for your solutions at [rust-learn-guide/solutions/11-testing.md](rust-learn-guide/solutions/11-testing.md).
2. Create online unit tests for your mathematical calculations inside your workspace, and verify that mock errors fail the assertions.
3. Execute `cargo test -p todo_cli` on the todo capstone project in the examples folders to verify that the existing tests pass cleanly.

### Exercise 2: Panic assertions block
Create a function `get_even_number(val: i32) -> i32` that panics if `val` is odd. Write a unit test using `#[should_panic]` that passes when the function is called with an odd number.

Now that we can confidently verify code correct behaves, let us explore collections and iterators. Go to [rust-learn-guide/chapters/12-collections-and-iterators.md](rust-learn-guide/chapters/12-collections-and-iterators.md).
