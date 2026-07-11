# Chapter 10: Organizing Code with Modules and Crates

As your applications grow larger, packing all of your code into `src/main.rs` becomes unmanageable. Rust provides a robust module system that lets you organize code into physical and logical compartments. 

This chapter explains how to group structured models using **Modules**, control item visibility with the **`pub` keyword**, split libraries into a multi-file file tree, and import packages using **Crates**.

---

## 1. Rust's Module System Terminology

To understand organization, let us define Rust's core structuring concepts:
- **Crates**: The basic compilation unit. A crate can be a **binary crate** (producing an executable binary) or a **library crate** (reusable source code intended for other projects).
- **Packages**: Built and managed by Cargo. A package consists of a `Cargo.toml` container and holds one or more crates.
- **Modules**: Let you group code (structs, functions, enums) logically within a single crate.
- **Paths**: The naming system you use to locate items (such as `std::collections::HashMap`).

---

## 2. Defining Modules with mod

You can bundle items into a virtual module using the `mod` keyword:

```rust
mod garden {
    // Everything in this module is private by default!
    fn water_plants() {
        println!("Watering the plants!");
    }

    // Explicitly make this function visible using 'pub'
    pub fn grow_vegetables() {
        println!("The carrots are growing!");
        water_plants(); // Private function is accessible inside the same module
    }
}

fn main() {
    // grow_vegetables is public, so we can access it using double-colon pathing
    garden::grow_vegetables();
    
    // garden::water_plants(); // Error: function `water_plants` is private!
}
```

---

## 3. Visibility: Public vs. Private

By default, everything in Rust (functions, structs, enums, fields, and constants) is **private**. This means it is only visible to the module in which it is defined, and any of its child inner modules.

To make an item visible to other modules, you must add the `pub` keyword before its declaration:

```rust
pub struct Task {
    pub title: String,  // This field is public
    done: bool,         // This field is private! Can only be changed inside Task methods
}

impl Task {
    pub fn new(title: String) -> Self {
        Task { title, done: false }
    }
}
```

---

## 4. Paths to Refer to Items in the Module Tree

To locate items inside your module tree, we write paths using double-colons:

1. **Absolute Paths**: Start from the root of the crate using the keyword `crate`:
   ```rust
   crate::garden::grow_vegetables();
   ```
2. **Relative Paths**: Start from the current module, or use `super` to navigate up to the parent directory:
   ```rust
   super::parent_function();
   ```
3. **The `use` Keyword**: You can bring paths into scope once to avoid typing out long paths repeatedly:
   ```rust
   use std::collections::HashMap;

   let mut map = HashMap::new(); // Now we use it directly!
   ```

---

## 5. Splitting Modules into Separate Files

When a module becomes large, you can move it into its own file.

Let us split our `garden` module out. In `src/main.rs`, we declare the module using `mod` without body brackets:

```rust
// In src/main.rs
mod garden; // Declares the module; Rust will look for src/garden.rs

fn main() {
    garden::grow_vegetables();
}
```

Then we create a companion file named `src/garden.rs` and write the module's contents directly inside it. We **do not** rewrite `mod garden` in the new file:

```rust
// In src/garden.rs
// No 'mod garden {' wrapping is needed here!

fn water_plants() {
    println!("Watering plants");
}

pub fn grow_vegetables() {
    println!("Growing carrots");
    water_plants();
}
```

---

## Exercises

### Exercise 1: Multi-File Modules
1. Read the matching solutions structured inside [rust-learn-guide/solutions/10-modules-and-crates.md](rust-learn-guide/solutions/10-modules-and-crates.md).
2. Inside one of your sandbox projects, create a module called `physics` inside a separate file `src/physics.rs`.
3. Declare a public function `calculate_velocity` inside it, and verify that you can call it from `src/main.rs`.

### Exercise 2: The use Shorthand
1. Bring `std::collections::HashMap` into scope inside a file using the `use` keyword.
2. Verify that you can instantiate mapping tables without double-colon prefixes.

Now that we can separate our code across files, let us learn how to write robust unit and integration tests. Head on to [rust-learn-guide/chapters/11-testing.md](rust-learn-guide/chapters/11-testing.md).
