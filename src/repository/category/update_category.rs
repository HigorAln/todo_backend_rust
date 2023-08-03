use super::category_repo::CategoryRepo;
use mongodb::{bson::doc, results::UpdateResult};
use rocket::http::Status;
use todo_backend::ResponseError;

impl CategoryRepo {
    pub fn update_category(
        &self,
        id: &String,
        name: &String,
    ) -> Result<UpdateResult, ResponseError> {
        let result = self
            .col
            .update_one(doc! {"_id": id}, doc! {"name": name}, None);

        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(ResponseError {
                status: Some(Status::InternalServerError),
                message: "Error updating category",
            }),
        }
    }
}
