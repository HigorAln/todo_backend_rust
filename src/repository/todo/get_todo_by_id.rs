use mongodb::bson::doc;
use todo_backend::ResponseError;

use crate::{models::todo_model::Todo, shared::validate_id::verify_object_id};

use super::todo_repo::TodoRepo;

impl TodoRepo {
    pub fn get_todo_by_id(&self, id: &String) -> Result<Todo, ResponseError> {
        let object_id = verify_object_id(id).unwrap();
        let find_todo = self.col.find_one(doc! {"_id": object_id}, None);

        match find_todo {
            Ok(v) => match v {
                Some(v) => Ok(v),
                None => Err(ResponseError {
                    status: Some(rocket::http::Status::NotFound),
                    message: "todo not found",
                }),
            },
            Err(_) => Err(ResponseError {
                status: Some(rocket::http::Status::InternalServerError),
                message: "we can't get a todo",
            }),
        }
    }
}
