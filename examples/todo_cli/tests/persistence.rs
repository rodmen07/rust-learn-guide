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
fn marks_tasks_done_and_persists_json() {
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

#[test]
fn lists_tasks_without_modifying_file() {
    let dir = temp_run_dir();
    let exe = env!("CARGO_BIN_EXE_todo_cli");

    let seed = serde_json::to_string_pretty(&[
        serde_json::json!({"title": "Write docs", "done": false}),
        serde_json::json!({"title": "Ship release", "done": true}),
    ])
    .expect("seed json");
    fs::write(dir.join("tasks.json"), &seed).expect("seed tasks.json");

    let output = Command::new(exe)
        .current_dir(&dir)
        .arg("list")
        .output()
        .expect("run todo_cli list");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("1: [todo] Write docs"));
    assert!(stdout.contains("2: [done] Ship release"));

    let after = fs::read_to_string(dir.join("tasks.json")).expect("read tasks.json");
    let left: serde_json::Value = serde_json::from_str(&after).expect("parse after");
    let right: serde_json::Value = serde_json::from_str(&seed).expect("parse seed");
    assert_eq!(left, right);
}

#[test]
fn removes_task_and_persists_json() {
    let dir = temp_run_dir();
    let exe = env!("CARGO_BIN_EXE_todo_cli");

    let seed = serde_json::to_string_pretty(&[
        serde_json::json!({"title": "Write docs", "done": false}),
        serde_json::json!({"title": "Ship release", "done": true}),
    ])
    .expect("seed json");
    fs::write(dir.join("tasks.json"), &seed).expect("seed tasks.json");

    let output = Command::new(exe)
        .current_dir(&dir)
        .arg("remove")
        .arg("1")
        .output()
        .expect("run todo_cli remove");

    assert!(output.status.success());

    let content = fs::read_to_string(dir.join("tasks.json")).expect("read tasks.json");
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("parse json");
    assert_eq!(parsed.as_array().unwrap().len(), 1);
    assert_eq!(parsed[0]["title"], serde_json::Value::String(String::from("Ship release")));
}