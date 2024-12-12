use crate::args::Args;
use crate::data::Priority::{High, Low, Medium, VeryHigh, VeryLow};
use chrono::Local;
use clap::ValueEnum;
use crossterm::style::Color;
use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::{
    error::Error,
    fmt::{Debug, Formatter},
    io::{self, Write},
};

#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub status: bool,
    pub time: String,
    pub priority: Priority,
}

pub fn get_input(text: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    print!("{text}");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

impl Todo {
    pub fn create(
        mut title: Option<String>,
        mut description: Option<String>,
        mut priority: Option<Priority>,
        id: u64,
    ) -> Result<(Todo, u64), Box<dyn Error>> {
        if title.is_none() {
            title = Some(get_input("Enter a title: ")?);
        }

        if description.is_none() {
            description = Some(get_input("Enter a description: ")?);
        }

        // Priority is an enum and we need to parse it
        if priority.is_none() {
            let mut input_needed = true;
            while input_needed {
                let input = get_input("Enter priority:\n(1) Very High\n(2) High\n(3) Medium\n(4) Low\n(5) Very Low\n=> ")?;

                priority = match input.as_str() {
                    "1" => Some(VeryHigh),
                    "2" => Some(High),
                    "3" => Some(Medium),
                    "4" => Some(Low),
                    "5" => Some(VeryLow),
                    _ => {
                        input_needed = true;
                        continue;
                    }
                };
                input_needed = false;
            }
        }
        Ok((
            Todo {
                title: title.unwrap(),
                description: description.unwrap(),
                status: false,
                time: Local::now().format("%Y-%m-%d %H:%M").to_string(),
                priority: priority.unwrap(),
            },
            id + 1,
        ))
    }
    pub fn display_tasks(&self, id: &str, highlight: bool) {
        let status: &str = if self.status { "Completed" } else { "Todo" };
        if highlight {
            println!(
                "{}",
                format!("({}) {} ({})", id, self.title, status)
                    .bold()
                    .with(Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255
                    })
            );
        } else {
            println!("({}) {} ({})", id, self.title, status);
        }
    }
}

impl Debug for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}, Description: {}, Priority: {:?}, time: {}, Status: {}",
            self.title, self.description, self.priority, self.time, self.status,
        )
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let status_message = if self.status {
            "Completed".green().to_string()
        } else {
            "Todo".red().to_string()
        };
        write!(
            f,
            "{} {}\n{} {}\n{} {}\n{} {}\n   - {}\n{}",
            "Title:".underlined().bold(),
            self.title,
            "Priority:".underlined().bold(),
            self.priority,
            "Time:".underlined().bold(),
            self.time.clone().bold(),
            "Status:".underlined().bold(),
            status_message,
            self.description,
            "-".repeat(30).bold(),
        )
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VeryHigh => "Very high".red(),
                High => "High".dark_yellow(),
                Medium => "Medium".yellow(),
                Low => "Low".green(),
                VeryLow => "Very low".blue(),
            }
        )
    }
}

#[derive(ValueEnum, Clone, Debug, Deserialize, Serialize)]
pub enum Priority {
    VeryHigh,
    High,
    Medium,
    Low,
    VeryLow,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub id: u64,
}

pub fn mark_task(
    completed: bool,
    todos: &mut BTreeMap<String, Todo>,
    args: &Args,
) -> Result<(), Box<dyn Error>> {
    let mut todo_title_id: String;
    if args.title.is_none() {
        loop {
            for todo in &mut *todos {
                if todo.1.status == completed {
                    todo.1.display_tasks(todo.0, true);
                } else {
                    todo.1.display_tasks(todo.0, false);
                }
            }
            let completed_todo = if completed {
                "n Uncompleted"
            } else {
                "Completed"
            };
            todo_title_id = get_input(
                format!("Select a{completed_todo} task (by id or title): ")
                    .bold()
                    .to_string()
                    .as_str(),
            )?;
            for (id, todo) in todos.iter_mut() {
                if todo_title_id == *id || todo_title_id == todo.title {
                    if todo.status == completed {
                        if todo.status {
                            println!("Select a task that's still todo");
                        } else {
                            println!("Select a task that's completed");
                        }

                        continue;
                    }
                    todo.status = completed;
                    if completed {
                        println!("Task \"{}\" completed", todo.title);
                    } else {
                        println!("Task \"{}\" todo", todo.title);
                    }
                    return Ok(());
                }
            }
        }
    } else {
        let mut found = false;
        for todo in todos.values_mut() {
            if Some(&todo.title) == args.title.as_ref() {
                found = true;
                todo.status = completed;
                if completed {
                    println!("Task \"{}\" completed", todo.title);
                } else {
                    println!("Task \"{}\" todo", todo.title);
                }
            }
        }
        if !found {
            println!("Task not found: {}", args.title.as_ref().unwrap());
        }
        Ok(())
    }
}
