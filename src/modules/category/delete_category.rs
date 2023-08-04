use rocket::{
    response::status::Custom,
    serde::json::{json, Json, Value},
};
use todo_backend::ResponseError;

use crate::{
    middleware::user::UserOnly, repository::category::category_repo::CategoryRepo,
    routes::category::verify_if_user_is_owner,
};

pub fn delete_category(
    id: String,
    user: UserOnly,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    let _ = verify_if_user_is_owner(user, &id);

    let result = CategoryRepo::init().delete_category(&id);

    match result {
        Ok(_) => Ok(Json(json!({ "message": "Category deleted successfully" }))),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
