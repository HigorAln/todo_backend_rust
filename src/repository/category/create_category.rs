use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json};
use todo_backend::ResponseError;

use crate::{
    models::category_model::Category, modules::category::create_category::CreateCategoryRequest,
};

use super::category_repo::CategoryRepo;

impl CategoryRepo {
    pub fn create_category(
        &self,
        id: String,
        data: Json<CreateCategoryRequest>,
    ) -> Result<InsertOneResult, ResponseError> {
        let category = Category {
            id: None,
            owner: ObjectId::parse_str(&id).unwrap(),
            name: data.name.to_owned(),
            todos: None,
        };

        let result = self.col.insert_one(category, None);

        match result {
            Ok(value) => Ok(value),
            Err(_) => Err(ResponseError {
                message: "We can't create category",
                status: Some(Status::InternalServerError),
            }),
        }
    }
}
