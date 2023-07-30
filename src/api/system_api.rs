use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{
    models::user_model::User,
    repository::{
        system_repo::{Login, SystemRepo},
        user_repo::UserRepo,
    },
};

#[get("/")]
pub fn health_check() -> Result<Json<String>, Custom<Json<ResponseError>>> {
    let system_repo = SystemRepo::init();
    let health_check = system_repo.health_check();

    match health_check {
        Ok(health_check) => Ok(Json(health_check)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}

#[post("/login", data = "<login>")]
pub fn login(login: Json<Login>) -> Result<Json<User>, Custom<Json<ResponseError>>> {
    let collection = UserRepo::init();
    let user = collection.login_user(&login.email, &login.password);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
