use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{response::status::Custom, serde::json::Json};

use crate::repository::user::user_repo::UserRepo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

pub fn login(login: Json<Login>) -> Result<Custom<Json<Value>>, Custom<Json<Value>>> {
    let collection = UserRepo::init();
    let user = collection.login_user(&login.email, &login.password);

    match user {
        Ok(login) => Ok(Custom(
            Status::Ok,
            Json(json!({
                "user": {
                    "id": login.user.id.map(|id| id.to_hex()).unwrap(),
                    "name": login.user.name,
                    "email": login.user.email,
                    "role": login.user.role,
                },
                "token": login.token,
            })),
        )),
        Err(_) => Err(Custom(
            Status::NotFound,
            Json(json!({
                "error": "e-mail or password invalid"
            })),
        )),
    }
}
