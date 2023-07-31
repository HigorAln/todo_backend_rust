use rocket::http::Status;

use crate::{middleware::admin::AdminOnly, repository::user::user_repo::UserRepo};

#[delete("/<id>")]
pub fn delete_user(id: String, _admin: AdminOnly) -> Result<Status, Status> {
    let collection = UserRepo::init();
    let delete_result = collection.delete_user(&id);

    match delete_result {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}
