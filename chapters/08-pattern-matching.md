# Pattern Matching

`match` lets you handle different cases clearly.

```rust
let direction = "north";

match direction {
    "north" => println!("Going up"),
    "south" => println!("Going down"),
    _ => println!("Unknown direction"),
}
```

Pattern matching is especially useful with enums and `Option`.

```rust
let value = Some(3);

match value {
    Some(number) => println!("{}", number),
    None => println!("nothing here"),
}
```

You can use `_` for a catch-all case and `if let` for simpler matches.

```rust
if let Some(number) = value {
    println!("{}", number);
}
```

## Exercise

1. Write a `match` on a number.
2. Handle at least three cases.
3. Add a default branch.

## Challenge

1. Match on an `Option` value.
2. Match on an enum of your choice.
3. Try one `if let` example.
