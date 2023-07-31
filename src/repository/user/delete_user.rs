use mongodb::{
    bson::{doc, oid::ObjectId},
    results::DeleteResult,
};
use rocket::http::Status;
use todo_backend::ResponseError;

use super::user_repo::{verify_object_id, UserRepo};

impl UserRepo {
    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, ResponseError> {
        let obj_id: ObjectId = match verify_object_id(&id) {
            Ok(obj) => obj,
            Err(err) => return Err(err),
        };

        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.delete_one(filter, None);

        match user_detail {
            Ok(user_detail) => Ok(user_detail),
            Err(_) => Err(ResponseError {
                message: "Error, not is possible delete this user",
                status: Some(Status::InternalServerError),
            }),
        }
    }
}
