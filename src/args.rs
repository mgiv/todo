use crate::data::Priority;
use clap::{Parser, ValueEnum};
#[allow(unused_imports)]
use SubCommand::*;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "Todo list app",
    arg_required_else_help = true
)]
pub struct Args {
    #[clap(value_enum)]
    /// Action to perform
    pub subcommand: SubCommand,

    /// Title of task
    pub title: Option<String>,

    /// Description (optional)
    pub description: Option<String>,

    #[clap(value_enum)]
    pub priority: Option<Priority>,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum SubCommand {
    /// List tasks
    Ls,

    /// Add a task by name
    Add,

    /// Complete a task
    Done,

    /// Leave a completed task incomplete
    Todo,

    /// Edit a task
    Edit,

    /// Remove a task
    Rm,

    /// Clean the to-do database (will prompt with confirmation)
    Clean,
}
