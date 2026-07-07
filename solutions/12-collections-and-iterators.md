# Solutions: Collections and Iterators

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let evens: Vec<i32> = numbers.into_iter().filter(|n| n % 2 == 0).collect();
    println!("{:?}", evens);
}
```