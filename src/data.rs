use crate::data::Priority::{High, Low, Medium, VeryHigh, VeryLow};
use crate::utils;
use chrono::Local;
use clap::ValueEnum;
use crossterm::style::Color;
use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Debug};

#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub status: bool,
    pub time: String,
    pub priority: Priority,
}

impl Todo {
    pub fn create(
        mut title: Option<String>,
        mut description: Option<String>,
        mut priority: Option<Priority>,
        id: u64,
    ) -> Result<(Todo, u64), Box<dyn Error>> {
        if title.is_none() {
            title = Some(utils::get_input("Enter a title: ")?);
        }

        if description.is_none() {
            description = Some(utils::get_input("Enter a description: ")?);
        }

        // Priority is an enum and we need to parse it
        if priority.is_none() {
            let mut input_needed = true;
            while input_needed {
                let input = utils::get_input("Enter priority:\n(1) Very High\n(2) High\n(3) Medium\n(4) Low\n(5) Very Low\n=> ")?;

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

#[derive(ValueEnum, Clone, Debug, Deserialize, Serialize)]
pub enum Priority {
    VeryHigh,
    High,
    Medium,
    Low,
    VeryLow,
}
