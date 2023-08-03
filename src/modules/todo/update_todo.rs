use mongodb::bson::oid::ObjectId;
use rocket::{
    response::status::Custom,
    serde::{
        json::{json, Json, Value},
        Deserialize, Serialize,
    },
};
use todo_backend::ResponseError;

use crate::repository::todo::todo_repo::TodoRepo;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub priority: Option<u8>,
}

pub fn update_todo(
    id: String,
    data: Json<UpdateTodoRequest>,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    let object_id = ObjectId::parse_str(&id).unwrap();

    let collection = TodoRepo::init();

    let update_result = collection.update_todo(object_id, data.into_inner());

    match update_result {
        Ok(_) => {
            let response = json!({
                "message": "Todo updated successfully"
            });

            Ok(Json(response))
        }
        Err(err) => return Err(Custom(err.status.unwrap(), Json(err))),
    }
}
