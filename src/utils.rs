use serde::{Deserialize, Serialize};
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