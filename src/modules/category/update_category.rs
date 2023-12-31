use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Json, Value},
};
use todo_backend::ResponseError;

use crate::{middleware::user::UserOnly, repository::category::category_repo::CategoryRepo};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
}

pub fn update_category(
    user: UserOnly,
    id: String,
    data: Json<UpdateCategoryRequest>,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    let result = CategoryRepo::init().update_category(&id, &data.name, &user.id);

    match result {
        Ok(_) => Ok(Json(json!({ "message": "Category updated successfully" }))),
        Err(err) => Err(Custom(
            err.status.unwrap_or(Status::InternalServerError),
            Json(err),
        )),
    }
}
