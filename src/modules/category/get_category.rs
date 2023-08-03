use rocket::{
    response::status::Custom,
    serde::json::{Json, Value},
};
use todo_backend::ResponseError;

use crate::repository::category::category_repo::CategoryRepo;

pub fn get_category(id: String) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    let collection = CategoryRepo::init();

    let result = collection.get_category(id);

    match result {
        Ok(category) => Ok(category),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
