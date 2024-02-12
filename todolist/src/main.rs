extern crate todolist;
use std::fmt::format;

use clap::Parser;
use sequelite::model::query::ColumnQueryFilterImpl;
use todolist::cli::main::Cli;
use todolist::db::init::init;
use todolist::todo::Todo;

use sequelite::connection::Executable;
use sequelite::model::ModelExt;

fn main() {
    let args = Cli::parse();
    let conn = init();
    match args.command.as_str() {
        "list" => {
            let todos = Todo::select().exec(&conn).unwrap();
            let todolist = todos
                .iter()
                .map(|todo: &Todo| {
                    let done = if todo.done { "x" } else { " " };
                    format(format_args!("[{}]\t{:?}\t{}", done, todo.id, todo.content))
                })
                .collect::<Vec<String>>()
                .join("\n");
            println!("{}", todolist)
        }
    }
}
