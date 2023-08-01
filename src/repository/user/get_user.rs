use mongodb::bson::{doc, oid::ObjectId};
use rocket::http::Status;
use todo_backend::ResponseError;

use crate::{models::user_model::User, shared::validate_id::verify_object_id};

use super::user_repo::UserRepo;

impl UserRepo {
    pub fn get_user(&self, id: String) -> Result<User, ResponseError> {
        let obj_id: ObjectId = match verify_object_id(&id) {
            Ok(obj) => obj,
            Err(err) => return Err(err),
        };

        let not_fount_error = ResponseError {
            message: "user not found",
            status: Some(Status::NotFound),
        };

        let filter = doc! {"_id": obj_id};
        let user = self.col.find_one(filter, None);

        let result = match user {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => return Err(not_fount_error),
            },
            Err(_) => Err(not_fount_error),
        };

        match result {
            Ok(user) => Ok(user),
            Err(err) => Err(err),
        }
    }
}
