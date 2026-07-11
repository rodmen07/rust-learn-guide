# Chapter 12: Collections and Iterators in Rust

Modern applications need to manipulate groups of values. While arrays and tuples provide structured indexing, they have fixed sizes. Rust offers dynamic, growable heap-allocated data structures called **Collections**, paired with **Iterators** to let you process and transform data step-by-step with clean, memory-safe abstractions.

This chapter covers the three most common standard collections (**Vectors**, **Strings**, and **HashMaps**), and the design of Rust's incredibly efficient, lazy-evaluated **Iterator pipeline**.

---

## 1. Core Collections of the Standard Library

### A. Vectors (`Vec<T>`)
A vector stores a list of values of the same type in a contiguous block of memory on the heap:

```rust
let mut v: Vec<i32> = Vec::new(); // Explicit initialization

// Using the vec! macro shorthand
let mut numbers = vec![1, 2, 3]; 

numbers.push(4); // Append an element
numbers.push(5);

// Accessing elements safely via get() which returns Option
match numbers.get(1) {
    Some(third) => println!("The second element is {}", third),
    None => println!("There is no second element."),
}
```

---

### B. Standard UTF-8 Strings (`String`)
Rust strings are represented as collections of UTF-8 characters. The standard `String` is dynamic and growable on the heap:

```rust
let mut s = String::from("hello");
s.push_str(" world"); // Append a string slice
s.push('!'); // Append a single character

// Concatenation using format macro
let combined = format!("My message is: {}", s);
```
*(Warning: Rust does not allow direct indexing on String variables like s[0], because UTF-8 characters can vary in size from 1 to 4 bytes. Indexing would run at unpredictable O(N) speeds, so Rust forces you to use slices or iterated char flows explicitly.)*

---

### C. Hash Maps (`HashMap<K, V>`)
A `HashMap` stores values mapped to unique keys of type `K` and values of type `V`, allowing rapid lookups inside O(1) average time complexities:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue Team"), 10);
scores.insert(String::from("Red Team"), 50);

// Retrieving matching stats safely
let team_name = String::from("Blue Team");
let score: Option<&i32> = scores.get(&team_name);

match score {
    Some(points) => println!("Score: {}", points),
    None => println!("Team not found"),
}
```

---

## 2. Understanding Iterators: The Lazy Pipeline

An iterator is a stateful object that lets you process a sequence of elements. In Rust, **iterators are lazy**. They do nothing and perform no computations until you call a terminal method that consumes them.

All iterators implement the standard `Iterator` trait, which requires defining a `next()` method:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

---

## 3. Adapters and Consumers

Managing data sequences involves chaining two classes of methods: **Iterator Adaptors** and **Consumers**.

### A. Iterator Adaptors (Transform layout)
Adaptors transform an iterator into another iterator, but because of lazy evaluation, they do not execute anything on their own. They simply build up a pipeline representation:

- **`map`**: Applies a closure or function to each element.
- **`filter`**: Filters elements keeping only those matching a condition.

```rust
let numbers = vec![1, 2, 3, 4, 5];

// This pipeline is lazy; nothing compiles or iterates yet!
let doubled_evens = numbers.iter()
    .filter(|num| *num % 2 == 0)
    .map(|num| num * 2);
```

### B. Consumers (Terminate and Execute)
Consumers are methods that execute and run the iterator state machine to produce a final result:

- **`sum`**: Combines elements by adding them.
- **`collect`**: Gathers elements into a concrete collection (like `Vec` or `HashMap`).

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Doubled even elements: 2 -> 4, 4 -> 8. Sum is 12.
let total: i32 = numbers.iter()
    .filter(|num| *num % 2 == 0)
    .map(|num| num * 2)
    .sum(); // Sum is a consuming method

println!("Total sum: {}", total); // Prints 12
```

Using `collect()` to compile results back into containers:
```rust
let base = vec![10, 20, 30];
let results: Vec<i32> = base.iter()
    .map(|x| x + 1)
    .collect(); // Collect resolves types and allocates memory
```

At compile time, Rust's optimizer unrolls these lazy iterator adapters into specialized raw loops. They compile down to the exact same speed as manual, hand-tuned assembly loops!

---

## Exercises

### Exercise 1: Mapping the Collection
1. Browse reference exercises inside [rust-learn-guide/solutions/12-collections-and-iterators.md](rust-learn-guide/solutions/12-collections-and-iterators.md).
2. Write a function that takes a reference to a vector of integers, keeps only the odd numbers, triples each integer, and returns a new `Vec<i32>`.
3. Verify that your tests pass cleanly.

### Exercise 2: Building key mapping tables
Create a `HashMap` mapping English words (as keys) to Spanish words. Perform lookups against at least two words, and return safe error messages if a key lookup is unsuccessful.

Now that we can process large amounts of data efficiently, let us investigate Rust's non-blocking I/O paradigm: Async Basics. Head on to [rust-learn-guide/chapters/13-async-basics.md](rust-learn-guide/chapters/13-async-basics.md).
