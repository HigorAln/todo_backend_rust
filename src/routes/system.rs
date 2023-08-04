use rocket::{
    response::status::Custom,
    serde::json::{Json, Value},
};
use todo_backend::ResponseError;

use crate::modules::system::login::Login;

#[get("/")]
pub fn health_check_router() -> Result<Json<String>, Custom<Json<ResponseError>>> {
    crate::modules::system::health_check::health_check()
}

#[post("/login", data = "<login>")]
pub fn login(login: Json<Login>) -> Result<Custom<Json<Value>>, Custom<Json<Value>>> {
    crate::modules::system::login::login(login)
}

pub fn system_routes() -> Vec<rocket::Route> {
    routes![health_check_router, login]
}
