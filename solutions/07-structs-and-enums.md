# Solutions: Structs and Enums

```rust
struct Book {
    title: String,
    author: String,
}

fn print_title(book: &Book) {
    println!("{}", book.title);
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let book = Book {
        title: String::from("The Rust Book"),
        author: String::from("Rust Team"),
    };

    print_title(&book);

    let light = TrafficLight::Green;
    match light {
        TrafficLight::Red => println!("stop"),
        TrafficLight::Yellow => println!("slow down"),
        TrafficLight::Green => println!("go"),
    }
}
```