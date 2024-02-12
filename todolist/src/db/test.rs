use super::init::init;
use crate::todo::Todo;
use sequelite::connection::Executable;
use sequelite::model::query::ColumnQueryFilterImpl;
use sequelite::model::ModelExt;

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

    // Get all todos whose content is "John"
    let todos = Todo::select()
        .filter(Todo::content.eq(&"asd"))
        .exec(&conn)
        .unwrap();

    // Print all todos
    println!("{:#?}", todos);
}
