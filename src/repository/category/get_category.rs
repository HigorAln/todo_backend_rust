use mongodb::bson::doc;
use rocket::serde::{Deserialize, Serialize};
use rocket::{
    http::Status,
    serde::json::{json, Json, Value},
};
use todo_backend::ResponseError;

use crate::{repository::todo::todo_repo::TodoRepo, shared::validate_id::verify_object_id};

use super::category_repo::CategoryRepo;

#[derive(Serialize, Deserialize)]
pub struct TodoResponse {
    _id: String,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub priority: Option<u8>,
    pub category: String,
}

impl CategoryRepo {
    pub fn get_category(&self, id: String) -> Result<Json<Value>, ResponseError> {
        dbg!(&id);
        let object_id = match verify_object_id(&id) {
            Ok(id) => id,
            Err(err) => {
                return Err(ResponseError {
                    message: err.message,
                    status: Some(err.status.unwrap()),
                })
            }
        };

        let filter = doc! {"_id": object_id};
        let result = self.col.find_one(filter, None).unwrap();

        match result {
            Some(doc) => {
                let todos_result = TodoRepo::init().get_todo_by_owner(id).unwrap();

                let todo_response: Vec<TodoResponse> = todos_result
                    .iter()
                    .map(|todo| TodoResponse {
                        _id: todo.id.map(|id| id.to_hex()).unwrap(),
                        title: todo.title.to_owned(),
                        description: todo.description.to_owned(),
                        done: todo.done,
                        priority: todo.priority,
                        category: todo.category.to_string(),
                    })
                    .collect();

                Ok(Json(json!({
                    "id": doc.id.map(|id| id.to_hex()).unwrap(),
                    "name": doc.name,
                    "todos": Some(todo_response),
                })))
            }
            None => {
                return Err(ResponseError {
                    message: "Category not found",
                    status: Some(Status::NotFound),
                })
            }
        }
    }
}
