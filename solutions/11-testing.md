# Solutions: Testing

```rust
fn double(value: i32) -> i32 {
    value * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_values() {
        assert_eq!(double(4), 8);
    }
}
```