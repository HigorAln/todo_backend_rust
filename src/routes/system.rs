use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{modules::system::login::Login, repository::user::login::LoginResponse};

#[get("/")]
pub fn health_check_router() -> Result<Json<String>, Custom<Json<ResponseError>>> {
    crate::modules::system::health_check::health_check()
}

#[post("/login", data = "<login>")]
pub fn login(login: Json<Login>) -> Result<Json<LoginResponse>, Custom<Json<ResponseError>>> {
    crate::modules::system::login::login(login)
}

pub fn system_routes() -> Vec<rocket::Route> {
    routes![health_check_router, login]
}
