use rocket::http::Status;

use crate::repository::user::user_repo::UserRepo;

pub fn delete_user(id: String) -> Result<Status, Status> {
    let collection = UserRepo::init();
    let delete_result = collection.delete_user(&id);

    match delete_result {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}
