use sequelite::prelude::Model;

#[derive(Debug, Model)]
pub struct Todo {
    pub id: Option<i32>,
    pub content: String,
    #[default_value(&false)]
    pub done: bool,
}
