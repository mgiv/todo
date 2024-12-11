// To-do list app
//
// Comments shall refer to to-do as to-do as my editor turns comments containing to-do (without
// hyphen) green
//
// Code related to clap is in args.rs, anything related to the structure
// is in data.rs, and everything related to writing to the file is in file.rs

use std::collections::BTreeMap;
use crate as todo;

use todo::{data::{ToDo, Config}, args::Args};

use clap::{Parser};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use dirs::cache_dir;
use crate::args::SubCommand::*;
use crate::file::{read_file, write_file};

mod args;
mod file;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut todos, mut id) = read_file()?;
    println!("{:#?}", todos);

    // Uses clap in args.rs
    let args = Args::parse();

    let mut todo: ToDo;

    if args.subcommand == Clean {
        let mut input = String::new();
        print!("Delete todo file? This will delete all todos (y/N): ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() == "y" {
            let mut path: PathBuf = match cache_dir() {
                Some(p) => p,
                None => return Err("Unable to access cache directory".into()),
            };
            path.push("todo");
            path.push("todo.toml");
            std::fs::remove_file(path)?;
            println!("All todos deleted");
        }
        return Ok(());
    }
    // If we need to fetch a specific to-do then create the struct (in data.rs).
    if args.subcommand != Ls || args.title.is_some() {

        (todo, id) = ToDo::create(args.title, args.description, args.priority, id)?;
        println!("{:?}", todo);
        todos.insert(id.to_string(), todo);
        let todos_valid = todos.into_iter()
            .map(|(k, v)| (k.to_string(), v))
        .collect::<BTreeMap<String, ToDo>>();

        write_file(todos_valid, Config { id })?;
    }
    Ok(())
}

