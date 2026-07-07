# Modules and Crates

Modules help organize code into smaller pieces.

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

A crate is a compilation unit. A binary crate produces an executable.

Modules can be split across files to keep code easier to navigate.

```rust
mod utils;

fn main() {
    println!("{}", utils::greet());
}
```

Public items are visible outside the module, while private items stay internal.

## Exercise

1. Create a module with one public function.
2. Call it from `main`.
3. Split related code into separate files.

## Challenge

1. Add a second module.
2. Re-export one function.
3. Keep the public API small.
