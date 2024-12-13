use crate::Todo;
use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::io;
use std::io::Write;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub id: u64,
}

pub fn get_input(text: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    print!("{text}");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn get_id(
    title: Option<String>,
    todos: &BTreeMap<String, Todo>,
) -> Result<String, Box<dyn Error>> {
    if title.is_none() {
        for todo in todos {
            todo.1.display_tasks(&todo.0, false);
        }
        let input = get_input("Select a task: ".bold().to_string().as_str())?;
        for (id, _todo) in todos.iter() {
            if *id == input {
                return Ok(id.to_string());
            }
        }
        Err("Not found".into())
    } else {
        return Ok(todos
            .iter()
            .find(|(_k, v)| v.title == *title.as_ref().unwrap())
            .ok_or("Todo not found")?
            .0
            .clone());
    }
}
