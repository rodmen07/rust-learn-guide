# Chapter 4a: References and Borrowing

In Chapter 4, we learned how ownership moves resources between variables. However, if we had to pass ownership back and forth for every simple print or mutation, Rust would be exhausting to write. To solve this, Rust uses a mechanism called **Borrowing**, enabling you to refer to a value without taking ownership of it.

---

## 1. What is borrowing?

A reference is like a pointer in C or C++: it points to the memory address of the owner variable. However, unlike raw pointers, Rust guarantees that references always point to valid memory for the entire duration of the reference.

We create a reference by using the ampersand `&` syntax:

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not own what it points to,
  // nothing happens. The underlying String is not dropped.

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // We pass a reference using &s1
    
    println!("The length of '{}' is {}.", s1, len); // s1 remains fully valid!
}
```

This act of creating a reference is called **borrowing**. The caller retains ownership, lending the data to the function.

---

## 2. The Two Rules of Borrowing

Rust enforces two strict rules at compile time to prevent data races and undefined behaviors:

1. **You can have any number of immutable references (`&T`) OR exactly one mutable reference (`&mut T`) at any given time, but never both.**
2. **References must always be valid (they cannot outlive their owners).**

Let us look at why these rules exist.

### Rule A: Immutable vs Mutable Borrows

An **immutable reference** (`&T`) lets you read data but not mutate it:
```rust
let s = String::from("hello");
let r1 = &s; // OK
let r2 = &s; // OK (multiple readers are perfectly safe)
```

A **mutable reference** (`&mut T`) lets you mutate the data:
```rust
let mut s = String::from("hello");
let r1 = &mut s; // OK
r1.push_str(" world");
```

However, if you mix them, the compiler will refuse to compile:
```rust
let mut s = String::from("hello");

let r1 = &s; // Problem starts here: immutable borrow
let r2 = &s; // OK
// let r3 = &mut s; // ERROR: cannot borrow s as mutable because it is also borrowed as immutable!

// println!("{}, {} and {}", r1, r2, r3);
```

#### Why does Rust prevent this?
If one section of your code has an immutable reference reading from a value, it expects that value to remain constant. If another thread or block mutated that memory using a mutable reference, the reader would experience undefined behavior or a crash. This compile-time guard completely eliminates **Data Races**.

---

## 3. Slices: References to a Contiguous Sequence

A slice is a type of reference that points to a contiguous sequence of elements in a collection rather than the entire collection.

### String Slices
A string slice is a reference to a portion of a `String`. We represent it as `&str`:

```rust
let s = String::from("hello world");

let hello: &str = &s[0..5]; // hello points to index 0 through 4 (inclusive of 0, exclusive of 5)
let world: &str = &s[6..11];

// Shorthand syntax
let hello = &s[..5]; // Starts at 0
let world = &s[6..]; // Goes to the end
let entire = &s[..]; // Slices the whole string
```

Slices are highly optimized because they simply store a starting pointer and a length.

---

## 4. Common Borrow Errors and How to Solve Them

As you write Rust, you will occasionally match wits with the **Borrow Checker**. Here are the most common compiler messages and how to resolve them:

### Pitfall A: "Cannot borrow as mutable because it is also borrowed as immutable"
*Cause*: You are holding an immutable reference while creating or using a mutable reference.
*Solution*: Ensure the scope of your immutable reference is finished before the mutable borrow begins.
```rust
// Error-prone
let mut list = vec![1, 2, 3];
let first = &list[0]; // Immutable borrow
list.push(4); // Error: cannot borrow as mutable!
println!("{}", first);

// Fixed: Limit scope or reorder
let mut list = vec![1, 2, 3];
list.push(4); // Mutable borrow occurs here and completes
let first = &list[0]; // Immutable borrow begins and ends here
println!("{}", first);
```

### Pitfall B: "Dangling References"
*Cause*: Trying to return a reference to a variable created inside a function. Once the function terminates, the local variable is dropped, leaving the returned reference pointing to invalid memory.
```rust
// ERROR: s goes out of scope and is dropped, so returning &s is illegal!
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s 
// }

// FIXED: Return the owned String instead to transfer ownership out!
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

---

## 5. Lifetimes: The Lifetime of a Reference

Every reference in Rust has a **lifetime**, which is the scope for which that reference is valid. Most of the time, the compiler can track and infer these lifetimes implicitly using a set of rules called "lifetime elision".

However, when a function accepts multiple references and returns a reference, the compiler cannot automatically deduce which input lifetime the return value belongs to. We must annotate this explicitly:

```rust
// 'a is a lifetime parameter. This tells the compiler that the returned slice
// will live at least as long as the shorter of the two input slices.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

## Exercises

### Exercise 1: Implement first_word_slice
1. Read the exercise solutions in [rust-learn-guide/solutions/borrow_examples.md](rust-learn-guide/solutions/borrow_examples.md) to inspect implementation patterns.
2. Implement your own version of `first_word_slice` in [rust-learn-guide/examples/borrow_examples/src/lib.rs](rust-learn-guide/examples/borrow_examples/src/lib.rs) that takes a string reference and returns its first word as a slice.
3. Test that it works against whitespace inputs.

### Exercise 2: Vector Mutations
Write a function that takes a mutable reference to a vector of integers (`&mut Vec<i32>`) and appends a value. Observe that the caller sees the mutated modifications directly inside their scope.

Next up, let us organize our code logic by studying functions in [rust-learn-guide/chapters/05-functions.md](rust-learn-guide/chapters/05-functions.md).
