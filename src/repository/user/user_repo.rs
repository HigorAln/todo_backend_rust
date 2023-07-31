use crate::models::user_model::User;
use mongodb::{
    bson::oid::ObjectId,
    sync::{Client, Collection},
};
use rocket::http::Status;
use todo_backend::ResponseError;

use dotenv::dotenv;
use std::env;

pub struct UserRepo {
    pub col: Collection<User>,
}

impl UserRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Erro, variável nao encontrada"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rust_api");
        let col: Collection<User> = db.collection("user");

        UserRepo { col }
    }
}

pub fn verify_object_id(id: &String) -> Result<ObjectId, ResponseError> {
    match ObjectId::parse_str(id) {
        Ok(obj) => return Ok(obj),
        Err(_) => {
            return Err(ResponseError {
                message: "This ID is not valid",
                status: Some(Status::BadRequest),
            })
        }
    }
}
