use crate::shared::validate_id::verify_object_id;

use super::category_repo::CategoryRepo;
use mongodb::{bson::doc, results::UpdateResult};
use rocket::http::Status;
use todo_backend::ResponseError;

impl CategoryRepo {
    pub fn update_category(
        &self,
        id: &String,
        name: &String,
        user_id: &String,
    ) -> Result<UpdateResult, ResponseError> {
        let object_id = match verify_object_id(id) {
            Ok(object_id) => object_id,
            Err(err) => return Err(err),
        };

        let new_doc = doc! {
            "$set": {
                "name": name
            }
        };

        let result =
            self.col
                .update_one(doc! { "_id": object_id, "owner": user_id }, new_doc, None);

        match result {
            Ok(result) => {
                if result.modified_count == 0 {
                    return Err(ResponseError {
                        status: Some(Status::NotFound),
                        message: "Anyone category was update",
                    });
                }
                Ok(result)
            }
            Err(_) => Err(ResponseError {
                status: Some(Status::InternalServerError),
                message: "Error updating category",
            }),
        }
    }
}
