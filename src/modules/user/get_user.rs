use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{models::user_model::User, repository::user::user_repo::UserRepo};

pub fn get_user(id: String) -> Result<Json<User>, Custom<Json<ResponseError>>> {
    let collection = UserRepo::init();
    let find_user = collection.get_user(id);

    match find_user {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
