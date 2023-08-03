use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Json, Value},
};
use todo_backend::ResponseError;

use crate::{repository::todo::todo_repo::TodoRepo, shared::validate_id::verify_object_id};

pub fn delete_todo(id: String) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    let object_id = match verify_object_id(&id) {
        Ok(id) => id,
        Err(_) => {
            return Err(Custom(
                Status::BadRequest,
                Json(ResponseError {
                    message: "Invalid ID",
                    status: None,
                }),
            ))
        }
    };

    let collection = TodoRepo::init();

    let delete_result = collection.delete_todo(object_id);

    match delete_result {
        Ok(_) => Ok(Json(json!({ "message": "Todo deleted successfully" }))),
        Err(e) => Err(Custom(
            e.status.unwrap_or(Status::NotFound),
            Json(ResponseError {
                message: "Error deleting todo",
                status: None,
            }),
        )),
    }
}
