use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Json, Value},
};
use todo_backend::ResponseError;

use crate::{
    middleware::user::UserOnly, repository::category::category_repo::CategoryRepo,
    routes::category::verify_if_user_is_owner,
};
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
    let _ = verify_if_user_is_owner(user, &id);

    let result = CategoryRepo::init().update_category(&id, &data.name);

    match result {
        Ok(_) => Ok(Json(json!({ "message": "Category updated successfully" }))),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json(ResponseError {
                status: Some(Status::InternalServerError),
                message: "Failed to update category",
            }),
        )),
    }
}
