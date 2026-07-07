# Ownership Basics

Ownership is Rust's core model for memory safety without a garbage collector.

## Key rules

- Each value has one owner.
- When the owner goes out of scope, the value is dropped.
- Values can be moved, borrowed, or copied depending on the type.

## Example

```rust
let text = String::from("hello");
let moved_text = text;
```

After the move, `text` can no longer be used.

Borrowing lets you read or update a value without taking ownership.

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

Mutable references let you change the borrowed value.

```rust
fn add_exclamation(text: &mut String) {
	text.push('!');
}
```

## Exercise

1. Create a `String`.
2. Pass it into a function by reference.
3. Print it before and after the call.

## Challenge

1. Write a function that takes a mutable reference.
2. Change the string inside the function.
3. Call it from `main` and print the result.
