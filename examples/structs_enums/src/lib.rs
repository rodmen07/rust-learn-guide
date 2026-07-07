use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Add(String),
    Remove(usize),
    List,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_new_and_mark_done() {
        let mut t = Task::new("write tests");
        assert_eq!(t.title, "write tests");
        assert!(!t.done);
        t.mark_done();
        assert!(t.done);
    }

    #[test]
    fn command_variants() {
        let c1 = Command::Add(String::from("task"));
        let c2 = Command::Remove(1);
        let c3 = Command::List;
        match c1 {
            Command::Add(s) => assert_eq!(s, "task"),
            _ => panic!("unexpected variant"),
        }
        if let Command::Remove(i) = c2 { assert_eq!(i, 1); }
        assert_eq!(c3, Command::List);
    }
}
