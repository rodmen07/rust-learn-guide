# borrow_examples

This tiny crate demonstrates borrowing rules and lifetimes used in the guide.

Examples:

Run tests:

```powershell
cargo test -p borrow_examples
```

Run the demo binary:

```powershell
cargo run -p borrow_examples
```

Functions included:

- `longest<'a>(a: &'a str, b: &'a str) -> &'a str` - returns the longest string slice.
- `append_value(v: &mut Vec<i32>, value: i32)` - shows a mutable borrow that mutates the caller's vector.
