use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub owner: ObjectId,
}
