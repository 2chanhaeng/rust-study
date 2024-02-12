extern crate todolist;
use std::fmt::format;

use clap::Parser;
use sequelite::model::query::ColumnQueryFilterImpl;
use todolist::cli::main::Cli;
use todolist::db::init::init;
use todolist::todo::Todo;

use sequelite::connection::Executable;
use sequelite::model::ModelExt;

fn help() {
    print!(
        "
- add < todo content > : add a todo
- list : show todos
- check < id >: check the todo
- remove < id >: remove the todo
- help : show this notice"
    )
}

fn main() {
    let args = Cli::parse();
    let conn = init();
    match args.command.as_str() {
        "add" => {
            let result = conn.insert(&[Todo {
                id: None,
                content: args.args.join(" "),
                done: false,
            }]);
            let todo = match result {
                Ok(id) => {
                    let todo = Todo::select().with_id(id).exec(&conn).unwrap();
                    format!("{:?}", todo)
                }
                _ => "Uh oh, something is wrong!".to_string(),
            };
            println!("{}", todo)
        }
        "check" => {
            let id = args.args[0].parse::<i64>().unwrap();
            let todo = Todo::update()
                .filter(Todo::id.eq(id))
                .set(Todo::done, true)
                .exec(&conn)
                .unwrap();
        }
        "remove" => {
            let id = args.args[0].parse::<i64>().unwrap();
            let todo = Todo::delete().filter(Todo::id.eq(id)).exec(&conn).unwrap();
        }
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
        "help" => {
            help();
        }
    }
}
