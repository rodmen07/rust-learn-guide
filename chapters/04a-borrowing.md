# Borrowing

Borrowing is the mechanism Rust uses to let you access data without taking ownership.

## Basic rules

- Use `&T` to borrow immutably — multiple immutable borrows allowed.
- Use `&mut T` to borrow mutably — only one mutable borrow at a time, and it cannot coexist with immutable borrows.
- Borrows are short-lived: they are tied to a lexical scope.

## Examples

Immutable borrow:

```rust
fn len(s: &String) -> usize {
    s.len()
}

let s = String::from("hello");
let l = len(&s);
println!("length: {}", l);
```

Mutable borrow:

```rust
fn append_exclamation(s: &mut String) {
    s.push('!');
}

let mut s = String::from("hi");
append_exclamation(&mut s);
println!("{}", s);
```

Borrowing slices:

```rust
fn first_word_slice(s: &str) -> &str {
    if let Some(i) = s.find(' ') {
        &s[..i]
    } else {
        s
    }
}

let s = String::from("hello world");
let fw = first_word_slice(&s);
println!("first word: {}", fw);
```

## Common borrow errors and fixes

- "cannot borrow `x` as mutable because it is also borrowed as immutable": remove the immutable borrow or limit its scope.
- "value moved here after partial borrow": avoid moving parts of a value while borrowing from it; use `clone()` only when acceptable.

## When to use references vs ownership

- Prefer `&T` when you only need to read data.
- Prefer `&mut T` when you need to modify an existing value.
- Take ownership (`T`) when you need to store or transfer data out of the caller's control.

## Brief intro to lifetimes

References in Rust have lifetimes — the scope during which the reference is valid. The compiler usually infers lifetimes for simple cases. When writing functions that return references, you may need to annotate lifetimes:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

Here `'a` ties the output lifetime to the shorter of the two input references.

## Exercises

1. Implement `first_word_slice` and test it with different strings.
2. Write a function that takes `&mut Vec<i32>` and appends a value; test that the caller sees the change.
3. Implement `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` and write tests demonstrating lifetime behaviour.

## Next

- Continue to Chapter 5 for functions and an emphasis on passing references to functions and returning owned vs borrowed values.
