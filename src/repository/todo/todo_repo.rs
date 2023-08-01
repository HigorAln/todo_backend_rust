use mongodb::sync::{Client, Collection};

use crate::models::todo_model::Todo;
use dotenv::dotenv;
use std::env;

pub struct TodoRepo {
    pub col: Collection<Todo>,
}

impl TodoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Erro, variável nao encontrada"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rust_api");
        let col: Collection<Todo> = db.collection("user");

        TodoRepo { col }
    }
}
