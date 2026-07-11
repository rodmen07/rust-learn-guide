# Chapter 4: The Pillars of Ownership

Ownership is Rust's most unique and defining feature. It enables Rust to make absolute guarantees about memory safety and resource management at compile time without relying on a garbage collector or manual deallocation. Understanding ownership is crucial for writing efficient and correct Rust software.

---

## 1. Why Ownership Exists: The Problem of Memory Management

Most programming languages manage memory in one of two ways:
1. **Garbage Collection (GC)**: The runtime system periodically scans memory to find and reclaim objects that are no longer referenced by the program (used in Go, Java, Python, and JavaScript). This is safe and convenient, but introduces non-deterministic execution pauses, larger memory footprints, and lack of fine-grained control.
2. **Manual Allocations**: The programmer must explicitly allocate memory when needed, and explicitly free it when finished (used in C and C++). If done perfectly, this is highly performant. However, it is extremely error-prone, leading to:
   - **Double Free Bugs**: Accidentally deallocating the same memory allocation twice, leading to crashes or security risks.
   - **Memory Leaks**: Forgetting to deallocate memory, causing the application to consume more and more system memory.
   - **Dangling Pointers**: Accessing memory through pointers that still reference locations that have already been cleared or reassigned.

**Rust takes a third path**: Memory is managed through a system of ownership with a set of rules evaluated by the compiler. If any of the rules are broken, your program simply fails to compile.

---

## 2. Stack vs Heap Memory

To understand ownership, it is helpful to appreciate the difference between the **Stack** and the **Heap**:

- **The Stack**: Fast, structured, and predictable. Data stored on the stack must have a fixed, known size at compile time. Stack memory operates in a last-in, first-out organization. It is incredibly quick because the CPU does not have to search for empty memory locations to place new inputs.
- **The Heap**: Dynamic and flexible. Data of unknown or mutable size at compile time (such as vectors or dynamic string inputs) must live on the heap. When you push data to the heap, you request a certain amount of space. The operating system allocates the space, marks it as in-use, and returns a **pointer** (which is a fixed-size address). The pointer sits on the stack, but the actual data lives on the heap.

Accessing heap data is slower than accessing stack data because you must follow the pointer to find the target.

---

## 3. The Rules of Ownership

The rules of ownership are strictly enforced block-by-block and line-by-line of your program:

1. **Each value in Rust has an owner (usually a variable).**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value is automatically dropped.**

### Scope and Resource Reclamation (RAII)
A scope is the range within a program for which an item is valid. Scopes are usually delimited by curly braces `{}`.

```rust
{
    // s is not declared yet; it is invalid here
    let s = String::from("hello"); // s is valid from this point forward
    
    // perform operations with s
} // this scope ends; s is no longer valid and is dropped!
```

When a variable goes out of scope, Rust calls a special function called `drop` on that variable. This is where the author of a type can place cleanup code (for example, deallocating heap memory or closing file handles). This pattern is known in systems programming as **RAII** (Resource Acquisition Is Initialization).

---

## 4. Memory Allocations and Data Behaviors

To see how Rust manages the heap without a garbage collector, let us analyze three fundamental data behaviors: **Move**, **Clone**, and **Copy**.

### A. Move (Shallow Copy with Invalidation)
When we assign one variable to another, what happens to the underlying data?

Let us look at a stack-allocated integer first:
```rust
let x = 5;
let y = x;
```
Because integers have a fixed size and are stored entirely on the stack, Rust copies the value. Both `x` and `y` are valid and bind to separate `5` values on the stack.

Now, let us try this with a heap-allocated `String`:
```rust
let s1 = String::from("hello");
let s2 = s1;
```

A `String` on the stack consists of three pieces of metadata:
1. **Pointer**: A reference pointing to the memory on the heap containing the actual characters `"hello"`.
2. **Length**: How much memory (in bytes) the contents are currently using.
3. **Capacity**: The total amount of memory the string has allocated.

When we assign `s1` to `s2`, Rust copies the metadata (pointer, length, capacity) from `s1` to `s2`. It **does not** copy the character data on the heap. 

If Rust did not intervene, this would create a major hazard: when both `s1` and `s2` go out of scope, they would both attempt to free the exact same heap memory address. This would trigger a **Double Free** crash!

To ensure memory safety, Rust invalidates `s1` the moment `s2` is created. This process is called a **Move**:

```rust
let s1 = String::from("hello");
let s2 = s1; // ownership has moved to s2!

// println!("{}", s1); // Error: value borrowed here after move!
```

---

### B. Clone (Deep Copy)
If we actually want to duplicate the heap contents of a variable, mudding our memory footprint with two separate heap allocations, we use the `clone` method:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // Deep copies the heap data!

println!("s1 = {}, s2 = {}", s1, s2); // Both are completely valid!
```

---

### C. Copy (Stack-Only Copy)
Why did our integer example not require a `clone`?

Types that have a size known at compile time are stored entirely on the stack. Rust has a special trait called `Copy`. If a type implements `Copy`, an older variable is not invalidated when assigned to a new one; it is simply duplicated via a rapid bitwise copy.

Any group of simple scalar values can implement `Copy`. Types that require heap allocation (such as `String`, `Vec`, or custom structs containing non-Copy properties) cannot implement `Copy`.

---

## 5. Ownership patterns and API design

When designing functions, ownership determines how callers interact with your functions.

- **Taking Ownership**: If a function takes a parameter by value, the caller surrenders ownership of that parameter.
  ```rust
  fn consume_string(s: String) {
      println!("Consuming: {}", s);
  } // s is dropped here! Heap allocation freed!

  fn main() {
      let message = String::from("hello");
      consume_string(message); // message is moved!
      // println!("{}", message); // Error! message can no longer be used!
  }
  ```

- **Returning Ownership**: If a function returns an owned type, the ownership of those resources is passed upstream to the caller:
  ```rust
  fn produce_string() -> String {
      let s = String::from("produced");
      s // s is returned without drop!
  }
  ```

To avoid losing ownership of variables every time we pass them into functions, we can borrow them. Let us learn about borrowing in the companion chapter: [rust-learn-guide/chapters/04a-borrowing.md](rust-learn-guide/chapters/04a-borrowing.md).

---

## Exercises

### Exercise 1: Take Ownership and Return
1. Open the file [rust-learn-guide/examples/borrow_examples/src/lib.rs](rust-learn-guide/examples/borrow_examples/src/lib.rs) and inspect the exercises.
2. Implement a function:
   ```rust
   pub fn take_and_return(s: String) -> String {
       // Append "!" to the string s and return it
       let mut s = s;
       s.push('!');
       s
   }
   ```
3. Test your configuration using your testing pipeline.

### Exercise 2: Ownership Pitfalls
Define a struct called `Note` containing a single `text` property of type `String`. Implement a method `into_text(self) -> String` that consumes the struct and returns the underlying string. Write a test case demonstrating that accessing the struct afterward is disallowed by the compiler. Check [rust-learn-guide/solutions/04-ownership.md](rust-learn-guide/solutions/04-ownership.md) for solutions.
