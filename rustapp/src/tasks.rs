use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

const FILE_NAME: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub done: bool,
}

pub fn load_tasks() -> Vec<Task> {
    match fs::read_to_string(FILE_NAME) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let data = serde_json::to_string(tasks)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_NAME)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn add_task(description: String) {
    let mut tasks = load_tasks();
    let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    tasks.push(Task {
        id,
        description,
        done: false,
    });
    save_tasks(&tasks).expect("Failed to save tasks.");
    println!("Task added successfully!");
}

pub fn list_tasks() {
    let tasks = load_tasks();
    for task in &tasks {
        println!(
            "[{}] {} - {}",
            task.id,
            if task.done { "x" } else { " " },
            task.description
        );
    }
}

pub fn mark_done(id: usize) {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.done = true;
        save_tasks(&tasks).expect("Failed to save tasks.");
        println!("Task marked as done!");
    } else {
        println!("Task not found.");
    }
}

pub fn remove_task(id: usize) {
    let mut tasks = load_tasks();
    let original_len = tasks.len();
    tasks.retain(|task| task.id != id);
    if tasks.len() < original_len {
        save_tasks(&tasks).expect("Failed to save tasks.");
        println!("Task removed!");
    } else {
        println!("Task not found.");
    }
}
