use mongodb::{
    bson::{doc, oid::ObjectId},
    results::UpdateResult,
};
use rocket::http::Status;
use todo_backend::ResponseError;

use crate::shared::validate_id::verify_object_id;

use super::user_repo::UserRepo;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserParams {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

impl UserRepo {
    pub fn update_user(
        &self,
        id: &String,
        params: UpdateUserParams,
    ) -> Result<UpdateResult, ResponseError> {
        let obj_id: ObjectId = match verify_object_id(&id) {
            Ok(obj) => obj,
            Err(err) => return Err(err),
        };

        let filter = doc! { "_id": obj_id};

        let mut update_fields = doc! {};

        if let Some(name) = params.name {
            update_fields.insert("name", name);
        }

        if let Some(email) = params.email {
            update_fields.insert("email", email);
        }

        if let Some(password) = params.password {
            update_fields.insert("password", password);
        }

        let new_doc = doc! {
            "$set": update_fields
        };

        let update_doc = self.col.update_one(filter, new_doc, None);

        match update_doc {
            Ok(update_doc) => Ok(update_doc),
            Err(_) => Err(ResponseError {
                message: "Error, not is possible update this user",
                status: Some(Status::InternalServerError),
            }),
        }
    }
}
