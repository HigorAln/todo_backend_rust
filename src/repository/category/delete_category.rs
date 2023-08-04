use mongodb::bson::doc;
use rocket::http::Status;
use todo_backend::ResponseError;

use crate::shared::validate_id::verify_object_id;

use super::category_repo::CategoryRepo;

impl CategoryRepo {
    pub fn delete_category(&self, id: &String, owner: &String) -> Result<(), ResponseError> {
        let object_id = match verify_object_id(&id) {
            Ok(object_id) => object_id,
            Err(err) => return Err(err),
        };

        let owner_object_id = match verify_object_id(&owner) {
            Ok(object_id) => object_id,
            Err(err) => return Err(err),
        };

        let result = self
            .col
            .delete_one(doc! { "_id": object_id, "owner": owner_object_id }, None);

        match result {
            Ok(deleted) => {
                if deleted.deleted_count == 0 {
                    return Err(ResponseError {
                        status: Some(Status::NotFound),
                        message: "Category not found",
                    });
                }

                Ok(())
            }
            Err(_) => Err(ResponseError {
                status: Some(Status::InternalServerError),
                message: "Failed to delete category",
            }),
        }
    }
}
