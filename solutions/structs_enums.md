# Structs and Enums - Solutions

## `Task` implementation

Use a simple named struct with `new` and `mark_done` methods:

```rust
pub struct Task {
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new<T: Into<String>>(title: T) -> Self {
        Task { title: title.into(), done: false }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}
```

## `Command` enum

Model CLI actions as an enum and match against variants in your CLI dispatch logic.

```rust
pub enum Command {
    Add(String),
    Remove(usize),
    List,
}
```

Testing tips: match on enum variants and assert contained data.
