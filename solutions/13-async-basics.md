# Solutions: Async Basics

```rust
async fn fetch_message() -> String {
    String::from("hello")
}

fn main() {
    let _future = fetch_message();
    println!("async functions return futures");
}
```