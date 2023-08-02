use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::repository::system::system_repo::SystemRepo;

pub fn health_check() -> Result<Json<String>, Custom<Json<ResponseError>>> {
    let system_repo = SystemRepo::init();
    let health_check = system_repo.health_check();

    match health_check {
        Ok(health_check) => Ok(Json(health_check)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
