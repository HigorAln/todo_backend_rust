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
    col: Collection<User>,
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

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            email: new_user.email.to_owned(),
            id: None,
            name: new_user.name.to_owned(),
            password: new_user.password.to_owned(),
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error create user");

        Ok(user)
    }
}
