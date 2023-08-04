use mongodb::{
    bson::{doc, oid::ObjectId},
    results::UpdateResult,
};
use rocket::http::Status;
use todo_backend::ResponseError;

use crate::modules::todo::update_todo::UpdateTodoRequest;

use super::todo_repo::TodoRepo;

impl TodoRepo {
    pub fn update_todo(
        &self,
        id: ObjectId,
        data: UpdateTodoRequest,
    ) -> Result<UpdateResult, ResponseError> {
        let filter = doc! { "_id": id};

        let mut update_fields = doc! {};

        if let Some(title) = data.title {
            update_fields.insert("title", title);
        }

        if let Some(description) = data.description {
            update_fields.insert("description", description);
        }

        if let Some(category) = data.category {
            update_fields.insert("category", category);
        }

        if let Some(priority) = data.priority {
            update_fields.insert("priority", priority as i32);
        }

        if let Some(done) = data.done {
            update_fields.insert("done", done);
        }

        let new_doc = doc! {
            "$set": update_fields
        };

        let update_doc = self.col.update_one(filter, new_doc, None);

        match update_doc {
            Ok(update_doc) => Ok(update_doc),
            Err(_) => {
                return Err(ResponseError {
                    message: "Error, not is possible update this todo",
                    status: Some(Status::InternalServerError),
                })
            }
        }
    }
}
