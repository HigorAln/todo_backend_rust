use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{models::todo_model::Todo, repository::todo::todo_repo::TodoRepo};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoRepo {
    title: String,
    description: Option<String>,
    category: String,
    priority: Option<u8>,
}

pub fn create_todo(
    data: Json<CreateTodoRepo>,
) -> Result<Json<InsertOneResult>, Custom<Json<ResponseError>>> {
    let todo: Todo = Todo {
        category: ObjectId::parse_str(&data.category).unwrap(),
        description: data.description.to_owned(),
        done: false,
        id: None,

        priority: data.priority,
        title: data.title.to_owned(),
    };

    let collection: TodoRepo = TodoRepo::init();
    let todo = collection.create_todo(todo);

    match todo {
        Ok(todo) => Ok(Json(todo)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
