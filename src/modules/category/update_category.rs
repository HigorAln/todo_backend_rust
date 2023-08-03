use rocket::{http::Status, serde::json::Json};
use todo_backend::ResponseError;

use crate::middleware::user::UserOnly;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
}

pub fn update_category(user: UserOnly, id: String, data: Json<UpdateCategoryRequest>) {
    let _ = verify_if_user_is_owner(user, &id);
}

fn verify_if_user_is_owner(user: UserOnly, id: &String) -> Result<(), ResponseError> {
    if user.id != *id {
        return Err(ResponseError {
            status: Some(Status::Unauthorized),
            message: "You are not the owner of this category",
        });
    }
    Ok(())
}
