# Ownership Basics

Ownership is Rust's core model for memory safety without a garbage collector.

## Key rules

- Each value has one owner.
- When the owner goes out of scope, the value is dropped.
- Values can be moved, borrowed, or copied depending on the type.

## Borrowing in practice

- Immutable borrows (`&T`) allow multiple simultaneous readers.
- Mutable borrows (`&mut T`) allow a single writer and cannot coexist with immutable borrows.
- You cannot move out of something while it's borrowed.

Example - immutable borrow:

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s; // multiple immutable borrows OK
println!("{} and {}", r1, r2);
```

Example - mutable borrow rules:

```rust
let mut s = String::from("hello");
{
	let r = &mut s; // one mutable borrow
	r.push_str(" world");
}
// r is out of scope here; we can borrow s again
println!("{}", s);
```

## Example

```rust
let text = String::from("hello");
let moved_text = text;
```

After the move, `text` can no longer be used.

## Ownership patterns and API design

- Prefer borrowing (`&T`, `&mut T`) in function APIs when the caller retains ownership.
- Accept `String` by value when your function needs to take ownership (e.g., to store it).
- For flexible APIs, consider `impl AsRef<str>` or `Into<String>` to accept multiple input types.

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

## Exercises - intermediate

1. Implement `fn take_and_return(s: String) -> String` that appends "!" and returns the value; write a unit test.
2. Implement `fn append_suffix(s: &mut String, suffix: &str)` and test it mutates the input.
3. Create a struct `Note { text: String }` with a method `fn into_text(self) -> String` that consumes `self` and returns the inner string. Add tests showing the consumed instance cannot be used.

## Common pitfalls

- Trying to use a moved value: the compiler will report that the value was moved.
- Confusing `Copy` types (small scalars) with owned heap types like `String` and `Vec`.

## Tips

- Prefer passing by reference for large data when you don't need ownership.
- Use `clone()` sparingly — prefer API design that avoids unnecessary cloning.

## Debugging borrow errors

- Read the compiler message: it usually points to the borrow and the conflicting use.
- Use small repros: reduce code to the smallest example that still shows the error.
- When in doubt, break complex expressions into separate `let` bindings so the borrow lifetimes end earlier.

## Extended exercises

1. Implement a small function `fn take_and_return(s: String) -> String` that takes ownership and returns the string with additional text appended. Write a test for this function.
2. Create a struct that owns a `String`, implement a method that takes `self` (consumes the struct) and returns its inner string.

## Next

- Chapter 5 explains lifetimes and how the borrow checker reasons about references across scopes.

## Challenge

1. Write a function that takes a mutable reference.
2. Change the string inside the function.
3. Call it from `main` and print the result.
