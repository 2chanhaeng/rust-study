use crate::todo::Todo;
use sequelite::prelude::Connection;

/// # Panics
///
/// Will panic if the database schema is not up to date
#[must_use]
pub fn init() -> Connection {
    // Create new database connection
    let mut conn = Connection::new("dev.db").unwrap();
    // Ensure database schema is up to date
    conn.register::<Todo>().unwrap();
    conn.migrate();
    conn
}
