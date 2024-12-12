use crate::Todo;
use dirs::cache_dir;
use std::collections::BTreeMap;
use toml::from_str;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    path::PathBuf,
};
use crate::utils::Config;

pub fn read() -> Result<(BTreeMap<String, Todo>, u64), Box<dyn Error>> {
    let mut file = open()?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    let toml_data: TomlData = from_str(&string)?;

    Ok((
        toml_data
            .todos,
        toml_data.config.id,
    ))
}

pub fn write(todos: BTreeMap<String, Todo>, config: Config) -> Result<(), Box<dyn Error>> {
    let toml = TomlData { config, todos };
    let toml_string = toml::to_string(&toml)?;
    let mut file = open()?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}


pub fn open() -> Result<File, Box<dyn Error>> {
    let mut path: PathBuf = match cache_dir() {
        Some(p) => p,
        None => return Err("Unable to access cache directory".into()),
    };

    path.push("todo");

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    path.push("todo.toml");

    let mut is_empty: bool = false;
    if path.exists() {
        let metadata = path.metadata()?;
        is_empty = metadata.len() == 0;
    }
    if !path.exists() || is_empty {
        let mut file_creator = File::create(&path)?;
        let default = toml::to_string(&TomlData {
            config: Config { id: 0 },
            todos: BTreeMap::new(),
        })?;
        file_creator.write_all(default.as_bytes())?;
    }

    Ok(OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?)
}

#[derive(Serialize, Deserialize, Debug)]
struct TomlData {
    config: Config,
    #[serde(flatten)]
    todos: BTreeMap<String, Todo>,
}
