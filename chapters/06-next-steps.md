# Next Steps

You now have a small Rust foundation. The best way to improve is to build small things and keep iterating.

## What to Practice

- variables, ownership, and borrowing
- functions with inputs and outputs
- simple control flow and matching
- reading compiler errors carefully

## Lifetimes and generics (next focus)

- Lifetimes describe how long references are valid. They help the compiler ensure references never outlive the data they point to.
- Generics let you write code that works for many types. Combine generics with trait bounds to express behaviour (e.g., `T: AsRef<str>`).

Quick lifetime example:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
	if a.len() >= b.len() { a } else { b }
}
```

Generic with trait bound example:

```rust
fn print_label<L: AsRef<str>>(label: L) {
	println!("{}", label.as_ref());
}
```

## Recommended reading order

1. Chapter 4 (Ownership) - ensure you can explain moves and borrows.
2. Chapter 4a (Borrowing) - practice `&T` and `&mut T`, slices, and short lifetimes.
3. Chapter 5 (Functions) - generics and passing/returning references.
4. Chapter 7 (Structs and Enums) - combine ownership and types.

## Suggested projects

- Extend the `todo_cli` with an `edit` subcommand and improve CLI ergonomics using `clap` features.
- Build a small library that exposes both borrowed and owned APIs and write tests demonstrating both.

## Exercises

1. Read Chapter 4a and implement `longest` and write tests showing lifetime constraints.
2. Create a function that accepts `impl AsRef<str>` and call it with both `&str` and `String`.

## Next

- Chapter 7 covers `struct`s and `enum`s with practical examples and exercises.

## Small Projects

Try one of these next:

- a calculator
- a unit converter
- a command-line todo list
- a file-backed notes app

## Tips

- Start with one feature at a time.
- Make it run before making it fancy.
- Add tests once the behavior is clear.

## Exercise

1. Pick one small project.
2. List the inputs and outputs.
3. Implement one working version.

## Challenge

1. Add tests to the project.
2. Save data to a file.
3. Improve the command-line messages.
