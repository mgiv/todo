use crate::data::Priority::{High, Low, Medium, VeryHigh, VeryLow};
use crate::data::{Priority, Todo};
use crossterm::style::Stylize;
use std::fmt::{Debug, Display, Formatter};

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
