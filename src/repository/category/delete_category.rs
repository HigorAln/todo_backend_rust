use mongodb::bson::doc;
use rocket::http::Status;
use todo_backend::ResponseError;

use super::category_repo::CategoryRepo;

impl CategoryRepo {
    pub fn delete_category(&self, id: &String) -> Result<(), ResponseError> {
        let result = self.col.delete_one(doc! { "_id": id }, None);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ResponseError {
                status: Some(Status::InternalServerError),
                message: "Failed to delete category",
            }),
        }
    }
}
