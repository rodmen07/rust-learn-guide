# join_names - Solution

Implementation uses `AsRef<str>` for ergonomics so callers can pass `&str` or `String`:

```rust
pub fn join_names<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> String {
    format!("{} {}", a.as_ref(), b.as_ref())
}
```

Tests should cover combinations of `&str` and `String` inputs.
