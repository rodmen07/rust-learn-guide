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

## Structs - examples and patterns

- Tuple structs, named-field structs, and unit structs each serve different needs.

Named struct example:

```rust
struct Task {
    title: String,
    done: bool,
}

impl Task {
    fn new<T: Into<String>>(title: T) -> Self {
        Task { title: title.into(), done: false }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }
}
```

## Enums - examples and patterns

- Use enums for variants and to encode domain state (e.g., `enum Command { Add, Remove(usize), List }`).

Example enum with data:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

## Exercises - structs & enums

1. Implement `Task` as above and write unit tests for `new` and `mark_done`.
2. Create an enum `Command` for a CLI (Add(String), Remove(usize), List) and write tests matching variants.

## Next

- Combine structs and enums in a small example `examples/structs_enums` that models tasks and commands; tests should cover behavior.

## Challenge

1. Add an author field to `Book`.
2. Create a constructor function.
3. Print the full book details.
