use mongodb::{
    bson::{doc, oid::ObjectId},
    results::DeleteResult,
};
use rocket::http::Status;
use todo_backend::ResponseError;

use super::todo_repo::TodoRepo;

impl TodoRepo {
    pub fn delete_todo(&self, id: ObjectId) -> Result<DeleteResult, ResponseError> {
        let result = self.col.delete_one(doc! {"_id": id}, None);

        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(ResponseError {
                message: "Error deleting todo",
                status: Some(Status::NotFound),
            }),
        }
    }
}
