use rocket::serde::{Deserialize, Serialize};
use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::repository::user::{login::LoginResponse, user_repo::UserRepo};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

pub fn login(login: Json<Login>) -> Result<Json<LoginResponse>, Custom<Json<ResponseError>>> {
    let collection = UserRepo::init();
    let user = collection.login_user(&login.email, &login.password);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
