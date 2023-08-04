use mongodb::bson::oid::ObjectId;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Json, Value},
};
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
) -> Result<Custom<Json<Value>>, Custom<Json<ResponseError>>> {
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
        Ok(todo) => {
            let id = todo.inserted_id.as_object_id().unwrap().to_hex();
            Ok(Custom(
                Status::Created,
                Json(json!({ "success": "Todo created successfully", "id": id })),
            ))
        }
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
