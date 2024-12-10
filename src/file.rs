use crate::{ToDo, data::Config};
use dirs::cache_dir;
use toml;
use std::collections::BTreeMap;

use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    path::PathBuf,
};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

pub fn read_file() -> Result<(BTreeMap<u64, ToDo>, u64), Box<dyn Error>> {
    let mut file = open_file()?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    let toml_data: TomlData = toml::from_str(&string)?;
    
    Ok((toml_data.todos, toml_data.config.id))
}

pub fn write_file(todos: BTreeMap<u64, ToDo>, config: Config) -> Result<(), Box<dyn Error>> {
    let toml = TomlData { config, todos };
    let toml_string = toml::to_string(&toml)?;
    let mut file = open_file()?;
    file.write(toml_string.as_bytes())?;
    Ok(())

}

pub fn open_file() -> Result<File, Box<dyn Error>> {
    let mut path: PathBuf = match cache_dir() {
        Some(p) => p,
        None => return Err("Unable to access cache directory".into()),
    };

    // Create parent dir if it doesn't exist
    path.push("todo");

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    path.push("todo.toml");

    Ok(OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?)
}

#[derive(Serialize, Deserialize, Debug)]
struct TomlData {
    config: Config,
    #[serde(flatten)]
    todos: BTreeMap<u64, ToDo>
    
}