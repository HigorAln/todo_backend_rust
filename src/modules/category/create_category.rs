use rocket::{
    http::Status,
    response::status::Custom,
    serde::{
        json::{json, Json, Value},
        Deserialize, Serialize,
    },
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
) -> Result<Custom<Json<Value>>, Custom<Json<ResponseError>>> {
    let collection = CategoryRepo::init();

    let result = collection.create_category(id.to_owned(), request);

    match result {
        Ok(category) => {
            let id = category.inserted_id.as_object_id().unwrap().to_hex();
            Ok(Custom(
                Status::Created,
                Json(json!({ "success": "Category created successfully", "id": id })),
            ))
        }
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
