use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    pub email: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}
