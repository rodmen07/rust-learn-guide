# Chapter 3: Variables, Mutability, and Types

In this chapter, you will learn how Rust represents, binds, and enforces data types. Rust is a strongly and statically typed language, meaning that every value has a concrete type known at compile time, preventing implicit conversions and unexpected runtime errors.

---

## 1. Variables and Mutability

By default, all variables in Rust are **immutable**. Once you bind a value to a name, that value cannot be changed.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // Error: cannot assign twice to immutable variable `x`
}
```

This immutability ensures safety. When multiple parts of your code access a variable, you can guarantee none of them will unexpectedly alter it behind your back.

### Declaring Mutable Variables
If you need to update a variable, you must explicitly opt in to mutability using the `mut` keyword:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is now: {}", x);
}
```

---

## 2. Shadowing

Rust allows you to declare a new variable with the exact same name as an existing one. This is called **shadowing**. When you shadow a variable, the new variable overrides (or shadows) the original variable within their overlapping scopes.

```rust
fn main() {
    let x = 5;
    let x = x + 1; // Shadows the first x
    {
        let x = x * 2; // Shadows x only inside this block scope
        println!("The inner value of x is: {}", x); // Prints 12
    }
    println!("The outer value of x is: {}", x); // Prints 6
}
```

### The Difference between mut and Shadowing
1. **Type Changes**: Shadowing lets us change the value type while retaining the name, whereas `mut` does not let us reallocate types.
   ```rust
   // OK: Shadowing changes type from &str to usize
   let spaces = "   ";
   let spaces = spaces.len();

   // ERROR: Re-assigning to mut variable must preserve type
   let mut spaces = "   ";
   // spaces = spaces.len(); // Type mismatch error!
   ```
2. **Re-binding**: Shadowing effectively creates a new immutable variable under the same name, preserving immutability after creation.

---

## 3. Data Types

Rust divides data types into two main categories: **Scalar Types** and **Compound Types**.

### A. Scalar Types
A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, booleans, and characters.

#### 1. Integers
An integer is a number without a fractional component. Integers are categorized by size and whether they support negative signs:

| Size | Signed | Unsigned |
| :--- | :--- | :--- |
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` (Default integer type) |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| Arch-dependent | `isize` | `usize` (Translates to the computer hardware's pointer width) |

#### 2. Floating-Point Numbers
Rust has two sizes of floating-point numbers: `f32` (single-precision float) and `f64` (double-precision float, default type because modern CPUs process it at the same speed as `f32` while offering higher accuracy).

```rust
let decimal: f64 = 3.14159;
```

#### 3. Booleans
Booleans represent a true-or-false value and are one byte in size:
```rust
let is_active: bool = true;
let is_finished = false; // inferred
```

#### 4. Characters
A `char` is Rust's basic alphabetic type and is four bytes in size. It represents a Unicode Scalar Value, supporting accents, Cyrillic, Chinese characters, and emoji:
```rust
let letter: char = 'R';
let emoji: char = '😻';
```

---

### B. Compound Types
Compound types can group multiple values together into a single structure. Rust has two primitive compound types: tuples and arrays.

#### 1. Tuples
A tuple is a general way of grouping together values of different types into one compound value. Tuples have a fixed length; once declared, they cannot grow or shrink.

```rust
let person: (&str, i32, bool) = ("Dan", 28, true);

// destructuring a tuple
let (name, age, active) = person;

// Accessing elements using dot notation
let person_name = person.0;
let person_age = person.1;
```

#### 2. Arrays
An array is a collection of multiple values of the same type. Arrays in Rust also have a fixed length and are allocated on the stack rather than the heap.

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];

// Initialize array with same value: [value; count]
let repeated_zeros = [0; 100]; // Creates array of 100 zeros

// Accessing elements using bracket indexing
let first = numbers[0];
let second = numbers[1];
```
*(Warning: if you attempt to access an out-of-bounds index, Rust will panic and terminate your program at runtime, preserving memory safety rather than returning garbage data as in C/C++.)*

---

## 4. Type Annotations and Inference

Rust is smart enough to infer the type of nearly any variable based on how you use it. For example, if you assign `5` to a variable, it automatically infers `i32`. 

However, there are times you must or should provide explicit type annotations:
1. When parsing strings to numeric types (since the compiler does not know what numeric size to parse into):
   ```rust
   let parsed_val: u32 = "42".parse().expect("Not a number");
   ```
2. When creating constants or static variables:
   ```rust
   const MAX_POINTS: u32 = 100_000;
   ```

---

## Exercises

### Exercise 1: Mutable Counter
We will write a small program to practice mutability constraints.
1. Open the source file at [rust-learn-guide/solutions/03-variables-and-types.md](rust-learn-guide/solutions/03-variables-and-types.md) to inspect solutions after completing your attempts.
2. Inside your Rust console or examples, perform the following steps:
   - Declare a mutable counter named `counter` and assign it a value of `0`.
   - Increment the counter by `10` across multiple lines.
   - Print the final value.

### Exercise 2: Destructure a Tuple
1. Declare an immutable tuple containing your city (as a string), population (as an integer), and whether it is a capital (as a boolean).
2. Access the city name using dot index syntax and print it.
3. Destructure the tuple to extract the population size and check your terminal outputs.

With these foundational concepts established, let us tackle Rust's defining feature: ownership and memory safety. Head over to [rust-learn-guide/chapters/04-ownership.md](rust-learn-guide/chapters/04-ownership.md).
