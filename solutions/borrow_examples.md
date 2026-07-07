# Borrowing Exercises - Solutions

This file contains suggested solutions and explanations for the exercises in `chapters/04a-borrowing.md`.

## `first_word_slice` / slices

A correct implementation uses `&str` slices and returns a borrowed slice tied to the input lifetime:

```rust
fn first_word_slice(s: &str) -> &str {
    if let Some(i) = s.find(' ') {
        &s[..i]
    } else {
        s
    }
}
```

Key points:
- The returned `&str` borrows from the input `s` so its lifetime is tied to the caller's scope.

## `append_value` (mutable borrow)

```rust
fn append_value(v: &mut Vec<i32>, value: i32) {
    v.push(value);
}
```

Test:

```rust
let mut v = vec![1,2];
append_value(&mut v, 3);
assert_eq!(v, vec![1,2,3]);
```

## `longest<'a>`

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Explanation:
- The lifetime parameter `'a` ties the returned reference to the lifetime of the smaller of `a` and `b` as enforced by the caller's usage.

## Notes
- Prefer returning owned `String` if you need the value to outlive borrowed data.
- When you see borrow-checker errors, try to reason about the lifetimes of each reference and shorten borrow scopes where possible.
