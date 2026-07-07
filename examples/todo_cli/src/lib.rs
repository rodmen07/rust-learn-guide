use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub done: bool,
}

pub fn parse_tasks(input: &str) -> Vec<Task> {
    serde_json::from_str(input).unwrap_or_default()
}

pub fn serialize_tasks(tasks: &[Task]) -> String {
    serde_json::to_string_pretty(tasks).unwrap_or_default()
}

pub fn format_tasks(tasks: &[Task]) -> Vec<String> {
    tasks
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let status = if task.done { "done" } else { "todo" };
            format!("{}: [{}] {}", index + 1, status, task.title)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{format_tasks, parse_tasks, serialize_tasks, Task};

    #[test]
    fn formats_tasks_with_status_and_index() {
        let tasks = vec![
            Task {
                title: String::from("Read the Rust book"),
                done: false,
            },
            Task {
                title: String::from("Build a small CLI"),
                done: true,
            },
        ];

        let formatted = format_tasks(&tasks);

        assert_eq!(formatted, vec![
            String::from("1: [todo] Read the Rust book"),
            String::from("2: [done] Build a small CLI"),
        ]);
    }

    #[test]
    fn round_trips_tasks() {
        let tasks = vec![
            Task {
                title: String::from("Read the Rust book"),
                done: false,
            },
            Task {
                title: String::from("Build a small CLI"),
                done: true,
            },
        ];

        let serialized = serialize_tasks(&tasks);
        let parsed = parse_tasks(&serialized);

        assert_eq!(parsed, tasks);
    }

    #[test]
    fn aliases_round_trip_helpers() {
        let tasks = vec![Task {
            title: String::from("Practice Rust"),
            done: true,
        }];

        let serialized = serialize_tasks(&tasks);
        let parsed = parse_tasks(&serialized);

        assert_eq!(parsed, tasks);
    }
}
