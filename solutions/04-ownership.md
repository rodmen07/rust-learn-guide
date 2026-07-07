# Solutions: Ownership Basics

```rust
fn print_text(text: &String) {
    println!("{}", text);
}

fn main() {
    let text = String::from("hello");
    print_text(&text);
    println!("{}", text);
}
```
