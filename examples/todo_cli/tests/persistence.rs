use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_run_dir() -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    dir.push(format!("todo_cli_test_{}", stamp));
    fs::create_dir_all(&dir).expect("create temp run dir");
    dir
}

#[test]
fn adds_and_persists_tasks_in_current_directory() {
    let dir = temp_run_dir();
    let exe = env!("CARGO_BIN_EXE_todo_cli");

    let seed = serde_json::to_string_pretty(&[serde_json::json!({"title": "Write docs", "done": false})]).expect("seed json");
    fs::write(dir.join("tasks.json"), seed).expect("seed tasks.json");

    let output = Command::new(exe)
        .current_dir(&dir)
        .arg("done")
        .arg("1")
        .output()
        .expect("run todo_cli");

    assert!(output.status.success());

    let tasks_file = dir.join("tasks.json");
    let content = fs::read_to_string(&tasks_file).expect("read tasks.txt");
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("parse json");
    assert_eq!(parsed[0]["done"], serde_json::Value::Bool(true));
}