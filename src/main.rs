use crate as todo;
use std::collections::BTreeMap;

use todo::{
    args::{Args, SubCommand},
    data::Todo,
    file::{read, write},
    utils::get_input
};

use crate::utils::get_id;
use clap::Parser;
use dirs::cache_dir;
use std::error::Error;
use std::io::Write;
use tasks::mark_task;
use utils::Config;

mod args;
mod data;
mod file;
mod tasks;
mod traits;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let (mut todos, mut id);
    if args.subcommand == SubCommand::Clean {
        (todos, id) = (BTreeMap::new(), 0)
    } else {
        (todos, id) = read()?;
    }
    let todo: Todo;

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
            (todo, id) = Todo::create(args.title, args.description, args.priority, id)?;
            todos.insert(id.to_string(), todo);
            let todos_valid = todos
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect::<BTreeMap<String, Todo>>();

            write(todos_valid, Config { id })?;
        }
        SubCommand::Edit => {
            let todo_id = get_id(args.title, &todos)?;
            for (id, todo) in &mut todos {
                if id == &todo_id {
                    let temp_time = todo.time.clone();
                    *todo = Todo::create(Some(todo.title.clone()), None, None, 0)?.0;
                    todo.time = temp_time.to_string();
                }
            }
            write(todos, Config { id })?;
        }
        SubCommand::Rm => {
            let todo_id = get_id(args.title, &todos)?;
            let mut todo_to_delete: Option<String> = None;
            for (id, todo) in  &todos {
                if id == &todo_id {
                    if get_input(&format!("Deleting: {}\nConfirm (y/N): ", todo.title).to_string())?.to_lowercase() == "y"{
                        todo_to_delete = Some(id.clone());
                        break;
                    }
                }
            }
            if todo_to_delete.is_some() {
                todos.remove::<String>(&todo_to_delete.unwrap());
                write(todos, Config { id })?;
                println!("Todo deleted")
            };
        }
        SubCommand::Done => {
            mark_task(true, &mut todos, &args)?;
            write(todos, Config { id })?;
        }
        SubCommand::Todo => {
            mark_task(false, &mut todos, &args)?;
            write(todos, Config { id })?;
        }
        SubCommand::Ls => {
            if args.title.is_none() {
                for todo in &todos {
                    println!("{}", todo.1);
                }
            } else {
                for todo in todos.values() {
                    if args.title.as_ref() == Some(&todo.title) {
                        println!("{todo}");
                    }
                }
            }
        }
    }
    Ok(())
}
