use super::init::init;
use crate::todo::Todo;
use sequelite::connection::Executable;
use sequelite::model::ModelExt;

/// # Panics
///
/// Will panic if the database schema is not up to date
pub fn main() {
    let conn = init();
    // Create a new todos
    let _ = conn.insert(&[
        Todo {
            id: None,
            content: "asd".to_string(),
            done: false,
        },
        Todo {
            id: None,
            content: "qwe".to_string(),
            done: false,
        },
    ]);

    let todos = Todo::select().exec(&conn).unwrap();

    // Print all todos
    println!("{todos:#?}");
}
