# Solutions: Error Handling

```rust
fn parse_positive(input: &str) -> Result<i32, String> {
    let value = input.parse::<i32>().map_err(|_| String::from("not a number"))?;

    if value < 0 {
        Err(String::from("value must be positive"))
    } else {
        Ok(value)
    }
}

fn main() {
    match parse_positive("5") {
        Ok(value) => println!("{}", value),
        Err(error) => println!("error: {}", error),
    }
}
```