// To-do list app
//
// Comments shall refer to to-do as to-do as my editor turns comments containing to-do (without
// hyphen) green
//
// Code related to clap is in args.rs, anything related to the structure
// is in data.rs, and everything related to writing to the file is in file.rs

use crate as todo;

use todo::data::ToDo;
use todo::args::Args;

use clap::{Parser};
use std::error::Error;
use crate::args::SubCommand::{*, self};
use crate::file::read_file;

mod args;
mod file;
mod data;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut todos, mut id) = read_file()?;
    println!("{:#?}", todos);

    // Uses clap in args.rs
    let args = Args::parse();

    let mut todo: ToDo;

    // If we need to fetch a specific to-do then create the struct (in data.rs).
    if args.subcommand != Ls || args.title.is_some() {

        todo = ToDo::create(args.title, args.description, args.priority)?;
        println!("{:?}", todo);
    }
    Ok(())
}

