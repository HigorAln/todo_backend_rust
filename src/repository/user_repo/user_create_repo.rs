use mongodb::results::InsertOneResult;
use rocket::http::Status;
use todo_backend::ResponseError;

use crate::models::user_model::User;

use super::user_repo::UserRepo;

impl UserRepo {
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, ResponseError> {
        let user = self.col.insert_one(new_user, None);

        match user {
            Ok(user) => Ok(user),
            Err(_) => Err(ResponseError {
                message: "Error, not is possible create this user",
                status: Some(Status::InternalServerError),
            }),
        }
    }
}
