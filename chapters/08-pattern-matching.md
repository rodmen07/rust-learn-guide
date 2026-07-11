# Chapter 8: Mastering Pattern Matching in Rust

One of Rust's most expressive and powerful control-flow structures is **Pattern Matching**. It allows you to compare a value against a series of patterns and execute code based on which pattern matches. This chapter covers the use of the `match` control flow, the compiler guarantee of match exhaustiveness, destructuring complex types, match guards, and the ergonomically streamlined `if let` syntax.

---

## 1. The Power of Match

A `match` block is like a multiple-choice branch. We define a value to inspect, and list one or more **match arms** comprised of a pattern and an expression to execute.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Match Exhaustiveness
In Rust, **matches are exhaustive**. This means you must cover every single possible choice or variant in your pattern arms. If you omit just one, your code will not compile.

```rust
// ERROR: non-exhaustive patterns: `Coin::Quarter` not covered!
// fn value_in_cents_incomplete(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//     }
// }
```
This is a tremendous safety feature. If you add a new variant to your enums later, the compiler will point to every single `match` statement across your codebase that needs updating!

---

## 2. Destructuring and Binding Values

Match arms can bind to the inner values held by enum variants or struct fields:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit command received");
        }
        Message::Move { x, y } => {
            println!("Move to position x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
    }
}
```

---

## 3. The Wildcard _ and Catch-All Patterns

When working with primitive integers, strings, or large enums, listing every possible value is impracticable. We can use wildcards or variable catch-alls.

### A. The Wildcard `_` (Ignore value)
The `_` pattern acts as a placeholder that matches any value but does not bind to it:

```rust
let roll = 9;
match roll {
    3 => println!("Lucky three!"),
    7 => println!("Lucky seven!"),
    _ => println!("Common roll"), // Matches everything else
}
```

### B. Variable Catch-All (Bind value)
If we want to match any value and also use that value in the match arm expression:
```rust
let roll = 9;
match roll {
    3 => println!("Lucky three!"),
    other => println!("You rolled a {}", other), // Binds value to 'other'
}
```

---

## 4. Match Guards

A **match guard** is an additional `if` condition specified after a pattern that must also match for that arm to execute. This lets you inspect values with fine-grained precision:

```rust
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("Even number: {}", x),
    Some(x) => println!("Odd number: {}", x),
    None => println!("No number"),
}
```

---

## 5. Ergonomic Control Flow with if let

In some cases, using a full `match` statement is overly verbose if you only want to handle a single variant. For example:

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("Max is {}", max),
    _ => (), // Redundant, boring boiler-plate
}
```

We can write this cleanly using **`if let`**:

```rust
let config_max = Some(3u8);

if let Some(max) = config_max {
    println!("Max is {}", max);
}
```

You can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and ignores everything else. You can also pair it with an `else` block:

```rust
if let Some(max) = config_max {
    println!("Max is {}", max);
} else {
    println!("No limit set");
}
```

---

## Exercises

### Exercise 1: Extract and Match Option Values
1. Check out exercise configurations at [rust-learn-guide/solutions/08-pattern-matching.md](rust-learn-guide/solutions/08-pattern-matching.md).
2. Write a function `unwrap_or_default` that accepts an `Option<i32>`. Use the `match` expression to return the inner value if it exists, or `0` if it is `None`.
3. Verify your implementation with a unit test or run command.

### Exercise 2: Destructure custom Coordinate Enums
Choose a coordinate enum representing either `2D(i32, i32)` or `3D(i32, i32, i32)`. Write an `if let` pattern that prints the positions only if it is in 2D space.

Let us now study how to safely propagation errors with the standard error-handling structures. Proceed to [rust-learn-guide/chapters/09-error-handling.md](rust-learn-guide/chapters/09-error-handling.md).
