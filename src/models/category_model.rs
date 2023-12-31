use mongodb::bson::oid::ObjectId;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,
    pub owner: ObjectId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub todos: Option<Vec<ObjectId>>,
}
