use rocket::{
    response::status::Custom,
    serde::json::{json, Json, Value},
};
use todo_backend::ResponseError;

use crate::{middleware::user::UserOnly, repository::category::category_repo::CategoryRepo};

pub fn delete_category(
    id: String,
    user: UserOnly,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    let result = CategoryRepo::init().delete_category(&id, &user.id);

    match result {
        Ok(_) => Ok(Json(json!({ "message": "Category deleted successfully" }))),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
