use dotenv::dotenv;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use std::env;
use todo_backend::ResponseError;

use mongodb::{
    bson::doc,
    sync::{Client, Collection},
};

use crate::models::user_model::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

pub struct SystemRepo {}

impl SystemRepo {
    pub fn init() -> SystemRepo {
        SystemRepo {}
    }

    pub fn health_check(&self) -> Result<String, ResponseError> {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Erro, variável nao encontrada"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("todo");
        let col: Collection<User> = db.collection("user");

        let result = col.find_one(doc! {}, None);

        match result {
            Ok(_) => Ok("Application is Running!".to_string()),
            Err(_) => Err(ResponseError {
                message: "Error, not is possible create this user",
                status: Some(Status::InternalServerError),
            }),
        }
    }
}
