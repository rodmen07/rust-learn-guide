# Structs and Enums

Structs group related data.

```rust
struct User {
    name: String,
    active: bool,
}
```

Enums let a value be one of several variants.

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

Structs can also have methods when you place behavior next to the data.

```rust
struct Book {
    title: String,
}

impl Book {
    fn print_title(&self) {
        println!("{}", self.title);
    }
}
```

Enums often carry data with each variant.

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```

## Exercise

1. Define a `Book` struct.
2. Add a function that prints its title.
3. Create an enum for traffic lights.

## Challenge

1. Add an author field to `Book`.
2. Create a constructor function.
3. Print the full book details.
