use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::serde::{Deserialize, Serialize};
use todo_backend::ResponseError;

use crate::models::todo_model::Todo;

use super::todo_repo::TodoRepo;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoRepo {
    title: String,
    description: Option<String>,
    category: String,
    priority: Option<u8>,
}

impl TodoRepo {
    pub fn create_todo(&self, data: CreateTodoRepo) -> Result<InsertOneResult, ResponseError> {
        let todo = Todo {
            category: ObjectId::parse_str(&data.category).unwrap(),
            description: data.description,
            done: false,
            id: None,

            priority: data.priority,
            title: data.title,
        };

        let todo_result = self.col.insert_one(todo, None);

        match todo_result {
            Ok(value) => Ok(value),
            Err(_) => Err(ResponseError {
                status: Some(rocket::http::Status::InternalServerError),
                message: "we can't create a todo",
            }),
        }
    }
}
