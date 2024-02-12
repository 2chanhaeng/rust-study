use crate::todo::Todo;
use sequelite::connection::Executable;
use sequelite::model::query::ColumnQueryFilterImpl;
use sequelite::model::ModelExt;
use sequelite::prelude::Connection;

pub fn main() {
    // Create new database connection
    let mut conn = Connection::new("dev.db").unwrap();

    // Ensure database schema is up to date
    conn.register::<Todo>().unwrap();
    conn.migrate();

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
