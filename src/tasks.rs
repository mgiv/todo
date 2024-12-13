use crate::args::Args;
use crate::data::Todo;
use crate::utils;
use crossterm::style::Stylize;
use std::collections::BTreeMap;
use std::error::Error;

pub fn mark_task(
    completed: bool,
    todos: &mut BTreeMap<String, Todo>,
    args: &Args,
) -> Result<(), Box<dyn Error>> {
    let mut todo_title_id: String;
    if args.title.is_none() {
        loop {
            for todo in &mut *todos {
                if todo.1.status == completed {
                    todo.1.display_tasks(todo.0, true);
                } else {
                    todo.1.display_tasks(todo.0, false);
                }
            }
            let completed_todo = if completed {
                "n Incomplete"
            } else {
                "Completed"
            };
            todo_title_id = utils::get_input(
                format!("Select a{completed_todo} task (by id or title): ")
                    .bold()
                    .to_string()
                    .as_str(),
            )?;
            for (id, todo) in todos.iter_mut() {
                if todo_title_id == *id || todo_title_id == todo.title {
                    if todo.status == completed {
                        if todo.status {
                            println!("Select a task that's still todo");
                        } else {
                            println!("Select a task that's completed");
                        }

                        continue;
                    }
                    todo.status = completed;
                    if completed {
                        println!("Task \"{}\" completed", todo.title);
                    } else {
                        println!("Task \"{}\" todo", todo.title);
                    }
                    return Ok(());
                }
            }
        }
    } else {
        let mut found = false;
        for todo in todos.values_mut() {
            if Some(&todo.title) == args.title.as_ref() {
                found = true;
                todo.status = completed;
                if completed {
                    println!("Task \"{}\" completed", todo.title);
                } else {
                    println!("Task \"{}\" todo", todo.title);
                }
            }
        }
        if !found {
            println!("Task not found: {}", args.title.as_ref().unwrap());
        }
        Ok(())
    }
}
