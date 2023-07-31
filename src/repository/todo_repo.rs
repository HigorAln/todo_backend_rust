use mongodb::{
    bson::oid::ObjectId,
    results::InsertOneResult,
    sync::{Client, Collection},
};
use todo_backend::ResponseError;

use crate::models::todo_model::Todo;
use dotenv::dotenv;
use std::env;

pub struct TodoRepo {
    col: Collection<Todo>,
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

    pub fn create_todo(
        &self,
        title: String,
        description: String,
        owner: String,
    ) -> Result<InsertOneResult, ResponseError> {
        let todo = Todo {
            id: None,
            title,
            description: Some(description),
            done: false,
            owner: ObjectId::parse_str(owner).unwrap(),
        };

        let todo_result = self.col.insert_one(todo, None);

        match todo_result {
            Ok(value) => Ok(value),
            Err(err) => Err(ResponseError {
                status: Some(rocket::http::Status::InternalServerError),
                message: "we can't create a todo",
            }),
        }
    }
}
