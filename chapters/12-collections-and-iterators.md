# Collections and Iterators

Vectors, strings, and hash maps are common Rust collections.

Iterators let you process data step by step.

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
```

Hash maps are useful when you need to look up values by key.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("blue", 10);
scores.insert("red", 20);
```

Iterators work well with `filter`, `map`, and `collect`.

## Exercise

1. Create a vector of numbers.
2. Use `map` to transform them.
3. Collect the result into a new vector.

## Challenge

1. Filter a list of numbers.
2. Build a hash map.
3. Sum a collection with an iterator.
