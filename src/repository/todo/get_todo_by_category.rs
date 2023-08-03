use crate::{models::todo_model::Todo, shared::validate_id};
use mongodb::bson::doc;
use todo_backend::ResponseError;

use super::todo_repo::TodoRepo;

impl TodoRepo {
    pub fn get_todo_by_owner(&self, id: String) -> Result<Vec<Todo>, ResponseError> {
        let object_id = match validate_id::verify_object_id(&id) {
            Ok(v) => v,
            Err(_) => {
                return Err(ResponseError {
                    status: Some(rocket::http::Status::BadRequest),
                    message: "invalid id",
                })
            }
        };

        let todo_result = self
            .col
            .find(doc! { "category": object_id }, None)
            .map_err(|_| ResponseError {
                status: Some(rocket::http::Status::InternalServerError),
                message: "we can't get a todo",
            })?;

        let todo_response: Vec<Todo> = todo_result.map(|todo| todo.unwrap()).collect();

        Ok(todo_response)
    }
}
