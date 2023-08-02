use dotenv::dotenv;
use rocket::http::Status;
use std::env;
use todo_backend::ResponseError;

use mongodb::{
    bson::doc,
    sync::{Client, Collection},
};

use crate::models::user_model::User;

use super::system_repo::SystemRepo;

impl SystemRepo {
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
