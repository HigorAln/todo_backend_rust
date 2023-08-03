use mongodb::sync::{Client, Collection};

use crate::models::category_model::Category;
use dotenv::dotenv;
use std::env;

pub struct CategoryRepo {
    pub col: Collection<Category>,
}

impl CategoryRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Erro, variável nao encontrada"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rust_api");
        let col: Collection<Category> = db.collection("category");

        CategoryRepo { col }
    }
}
