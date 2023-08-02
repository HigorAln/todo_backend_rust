use mongodb::results::InsertOneResult;
use todo_backend::ResponseError;

use crate::models::todo_model::Todo;

use super::todo_repo::TodoRepo;

impl TodoRepo {
    pub fn create_todo(&self, data: Todo) -> Result<InsertOneResult, ResponseError> {
        let todo_result = self.col.insert_one(data, None);

        match todo_result {
            Ok(value) => Ok(value),
            Err(_) => Err(ResponseError {
                status: Some(rocket::http::Status::InternalServerError),
                message: "we can't create a todo",
            }),
        }
    }
}
