use crate::models::user_model::User;
use mongodb::{
    error::Error,
    results::InsertOneResult,
    sync::{Client, Collection},
};

use std::env;
extern crate dotenv;
use dotenv::dotenv;

pub struct MongoRepo {
    pub col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Erro, variável nao encontrada"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rust_api");
        let col: Collection<User> = db.collection("user");
        MongoRepo { col }
    }
}
