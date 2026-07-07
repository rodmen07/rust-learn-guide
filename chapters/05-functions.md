# Functions

Functions let you organize behavior into reusable units.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Return values are explicit, and the last expression is returned without `return`.

Functions can also take multiple parameters and return different types of values.

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn square(value: i32) -> i32 {
    value * value
}
```

Variables declared inside a function only exist in that function's scope.

```rust
fn main() {
    let message = "inside main";
    println!("{}", message);
}
```

## Exercise

1. Write a function that doubles a number.
2. Call it from `main`.
3. Print the returned value.

## Challenge

1. Write a function that greets a person by name.
2. Write a function that squares a number.
3. Call both from `main`.
