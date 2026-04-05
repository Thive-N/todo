use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "todolist")]
#[command(about = "Simple CLI todo list", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        task: String,
        #[arg(short, long)]
        priority: Option<String>,
        #[arg(short, long)]
        due: Option<String>,
    },
    List,
    Done {
        index: usize,
    },
    Remove {
        index: usize,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Priority {
    High,
    Medium,
    Low,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Low
    }
}

impl Priority {
    fn from_string(s: Option<String>) -> Self {
        match s.unwrap_or_default().to_lowercase().as_str() {
            "high" => Priority::High,
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            _ => Priority::Low,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    task: String,
    done: bool,
    due_date: Option<String>,
    priority: Priority,
}

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let mut path = PathBuf::from(home);
    path.push(".config/todolist.json");
    path
}

fn load_todos() -> Vec<Todo> {
    let path = config_path();
    if !path.exists() {
        return vec![];
    }

    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

fn save_todos(todos: &Vec<Todo>) {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize");
    fs::write(path, data).expect("Failed to write file");
}

fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    match cli.command {
        Commands::Add {
            task,
            priority,
            due,
        } => {
            let todo = Todo {
                task,
                done: false,
                due_date: due,
                priority: Priority::from_string(priority),
            };
            todos.push(todo);
            save_todos(&todos);
            println!("Task added.");
        }
        Commands::List => {
            for (i, todo) in todos.iter().enumerate() {
                let status = if todo.done { "✔" } else { " " };
                let due = todo
                    .due_date
                    .as_ref()
                    .map(|d| d.as_str())
                    .unwrap_or("No due date");
                let priority = match todo.priority {
                    Priority::High => "High",
                    Priority::Medium => "Medium",
                    Priority::Low => "Low",
                };

                println!(
                    "{}: [{}] {} (Priority: {}, Due: {})",
                    i, status, todo.task, priority, due
                );
            }
        }
        Commands::Done { index } => {
            if let Some(todo) = todos.get_mut(index) {
                todo.done = true;
                save_todos(&todos);
                println!("Task marked as done.");
            } else {
                println!("Invalid index");
            }
        }
        Commands::Remove { index } => {
            if index < todos.len() {
                todos.remove(index);
                save_todos(&todos);
                println!("Task removed.");
            } else {
                println!("Invalid index");
            }
        }
    }
}
