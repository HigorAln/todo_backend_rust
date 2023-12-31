use rocket::{
    response::status::Custom,
    serde::json::{Json, Value},
};
use todo_backend::ResponseError;

use crate::{
    middleware::user::UserOnly, repository::category::category_repo::CategoryRepo,
    routes::category::verify_if_user_is_owner,
};

pub fn get_category(
    user: UserOnly,
    id: String,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    let _ = verify_if_user_is_owner(user, &id);
    let collection = CategoryRepo::init();

    let result = collection.get_category(id);

    match result {
        Ok(category) => Ok(category),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
