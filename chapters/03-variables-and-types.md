# Variables and Types

Rust variables are immutable by default.

```rust
let name = "Rust";
let mut score = 0;
score += 1;
```

You can add a type annotation when the compiler needs a hint.

```rust
let age: u32 = 10;
let pi: f64 = 3.14;
let ready: bool = true;
```

Common types include integers, booleans, floats, chars, tuples, and arrays.

```rust
let letter: char = 'R';
let point: (i32, i32) = (3, 7);
let numbers = [1, 2, 3];
```

## Key Ideas

- Variables are immutable unless you use `mut`.
- Type inference usually works without annotations.
- Shadowing lets you reuse a name with a new value.

```rust
let value = 5;
let value = value + 1;
```

## Exercise

1. Declare a mutable counter.
2. Increase it by 5.
3. Print the result.

## Challenge

1. Create a tuple with your name and age.
2. Print both values.
3. Try changing one variable from immutable to mutable.
