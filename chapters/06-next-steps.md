# Chapter 6: Synthesis and Midpoint Review

Congratulations! You have reached a pivotal milestone. You now understand variables, strict static typing, variables mutability, ownership, borrowing, and functional declarations. These are the deep roots of Rust.

In this chapter, we will synthesize what you have learned so far, review best practices for writing clean Rust APIs, and preview how we will construct custom types using structs and enums.

---

## 1. Midpoint Concept Checklist

Before moving forward, ensure you feel comfortable explaining the following questions:
- **Why are variables immutable by default?** (To enable compile-time safety and painless concurrency.)
- **What is the difference between a Move and a Copy?** (A Move transfers ownership of heap metadata and invalidates the source, while a Copy duplicates a stack-allocated scalar value.)
- **Why can you only have one mutable reference at a time?** (To guarantee that no other readers or writers can access the data concurrently, preventing data races.)
- **What is an expression?** (A piece of code that evaluates to a value and does not end with a semicolon.)

---

## 2. Best Practices for Rust API Design

By using borrowing and advanced types, you can design functions that are safe, quick, and comfortable to call:

- **Borrow by default**: Do not force callers to surrender their variables if you only need to read them. Prefer `&String` or `&str` over `String` parameter signatures.
- **Prefer Slices (`&str`, `&[T]`)**: Slices are more flexible than fully owned objects. If you accept a `&str`, callers can pass dynamic string slices, static strings, or references to parts of string variables.
- **Use `AsRef` and `Into` to minimize allocations**: By leveraging generics and trait bounds, you can write code that allows callers to pass either borrowed or owned values, allocating memory only when absolutely necessary.

---

## 3. Recommended Small Exercises

To cement these foundations, we recommend practicing these exercises:
1. Ensure your solution implementations are clean and completely verified. Check [rust-learn-guide/solutions/05-functions.md](rust-learn-guide/solutions/05-functions.md) for feedback.
2. Build a small, console-based unit converter (e.g., Celsius to Fahrenheit) inside your sandbox directories.

---

Let us now unlock Rust's full expressive power by learning how to declare custom types. Head over to [rust-learn-guide/chapters/07-structs-and-enums.md](rust-learn-guide/chapters/07-structs-and-enums.md).
