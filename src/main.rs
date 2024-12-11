// To-do list app
//
// Comments shall refer to to-do as to-do as my editor turns comments containing to-do (without
// hyphen) green
//
// Code related to clap is in args.rs, anything related to the structure
// is in data.rs, and everything related to writing to the file is in file.rs

use crate as todo;
use std::collections::BTreeMap;

use todo::{
    args::Args,
    data::{Config, ToDo},
};

use crate::args::SubCommand;
use crate::file::{read, write};
use clap::Parser;
use dirs::cache_dir;
use std::error::Error;
use std::io::Write;

mod args;
mod data;
mod file;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut todos, mut id) = read()?;

    // Uses clap in args.rs
    let args = Args::parse();

    let todo: ToDo;

    match args.subcommand {
        SubCommand::Clean => {
            let mut input = String::new();
            print!("Delete todo file? This will delete all todos (y/N): ");
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" {
               let cache_path = if let Some(p) = cache_dir() {
                p.join("todo").join("todo.toml")
               } else {
                   return Err("Unable to access cache directory.".into());
               };

                std::fs::remove_file(cache_path)?;
                println!("All todos deleted");
            }
            return Ok(());
        }
        SubCommand::Add => {
            (todo, id) = ToDo::create(args.title, args.description, args.priority, id)?;
            todos.insert(id.to_string(), todo);
            let todos_valid = todos
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect::<BTreeMap<String, ToDo>>();

            write(todos_valid, Config { id })?;
        }
        SubCommand::Edit => {unimplemented!()}
        SubCommand::Rm => {unimplemented!()}
        SubCommand::Done => {
            let todo_title_id: String;
            if args.title.is_none() {
                let mut counter = 0;
                for todo in todos {
                    todo.1.display_tasks(todo.0);
                    counter += 1;
                }
                loop {
                    todo_title_id = data::get_input("Select a task: ")?;

            }
        } else {
                for todo in todos.iter() {

                }
            }

        }
        SubCommand::ToDo => {unimplemented!()}
        SubCommand::Ls => {
            for todo in todos {
                println!("{}", todo.1);
            }
        }
    }
    Ok(())
}
