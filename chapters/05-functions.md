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

## Passing references and ownership

- Use `&T` to borrow data you don't need to own in the function.
- Use `&mut T` when the function should modify the caller's data.
- Take `T` by value when your function needs to take ownership (for example, to store it).

Example - borrowing vs ownership:

```rust
fn greet_borrow(name: &str) {
    println!("Hello, {}", name);
}

fn take_ownership(s: String) {
    println!("Stored: {}", s);
}

let name = String::from("Alex");
greet_borrow(&name);
take_ownership(name);
// name is no longer available here
```

## Generic parameter conveniences: `AsRef` and `Into`

- `AsRef<str>` lets you accept `&String` and `&str` without forcing a `String` allocation.
- `Into<String>` lets callers pass `&str` or `String`, consuming or cloning as appropriate.

Example - `AsRef` ergonomic API:

```rust
fn print_label<L: AsRef<str>>(label: L) {
    println!("Label: {}", label.as_ref());
}

print_label("inline");
print_label(String::from("owned"));
```

## Testing functions

- Write unit tests next to the code using `#[cfg(test)]` and `#[test]`.
- Test both borrowed and owned APIs where relevant.

## Exercises - advanced

1. Implement `fn join_names<A: AsRef<str>>(a: A, b: A) -> String` that returns `"a b"`.
2. Write tests for `join_names` accepting both `&str` and `String` inputs.
3. Refactor an earlier example to use `AsRef<str>` where it improves ergonomics.

## Challenge

1. Write a function that greets a person by name.
2. Write a function that squares a number.
3. Call both from `main`.
