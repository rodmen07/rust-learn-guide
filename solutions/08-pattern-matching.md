# Solutions: Pattern Matching

```rust
fn main() {
    let number = 2;

    match number {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),
    }

    let value = Some(7);

    if let Some(number) = value {
        println!("{}", number);
    }
}
```