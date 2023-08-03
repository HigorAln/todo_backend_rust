use mongodb::results::InsertOneResult;
use rocket::{
    response::status::Custom,
    serde::{json::Json, Deserialize, Serialize},
};
use todo_backend::ResponseError;

use crate::repository::category::category_repo::CategoryRepo;

#[derive(Deserialize, Serialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

pub fn create_category(
    id: String,
    request: Json<CreateCategoryRequest>,
) -> Result<Json<InsertOneResult>, Custom<Json<ResponseError>>> {
    let collection = CategoryRepo::init();

    let result = collection.create_category(id.to_owned(), request);

    match result {
        Ok(category) => Ok(Json(category)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
