use crate::data::Priority::{High, Low, Medium, VeryHigh, VeryLow};
use chrono::Local;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{
    error::Error,
    fmt::{Debug, Formatter},
    io::{self, Write},
};

#[derive(Deserialize, Serialize)]
pub struct ToDo {
    title: String,
    description: String,
    status: bool,
    time: String,
    priority: Priority,
}

pub fn get_input(text: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    print!("{text}");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

impl ToDo {
    pub fn create(
        mut title: Option<String>,
        mut description: Option<String>,
        mut priority: Option<Priority>,
        id: u64,
    ) -> Result<(ToDo, u64), Box<dyn Error>> {
        // Set title, description, and priority if they don't exist, get input and strip whitespace
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
            ToDo {
                title: title.unwrap(),
                description: description.unwrap(),
                status: false,
                time: Local::now().format("%Y-%m-%d %H:%M").to_string(),
                priority: priority.unwrap(),
            },
            id + 1,
        ))
    }
    pub fn display_tasks(&self, id: String) {
        println!("({}) {}", id, self.title);
    }
}

impl Debug for ToDo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}, Description: {}, Priority: {:?}, time: {}, Status: {}",
            self.title, self.description, self.priority, self.time, self.status,
        )
    }
}

impl Display for ToDo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}\nPriority: {}\nTime: {}\nCompleted: {}\n   - {}",
            self.title, self.priority, self.time, self.status, self.description
        )
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VeryHigh => "Very high",
                High => "High",
                Medium => "Medium",
                Low => "Low",
                VeryLow => "Very low",
            }
        )
    }
}

// Can be set from args or also from To-Do::create
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
