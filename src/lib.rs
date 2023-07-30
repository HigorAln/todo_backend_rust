use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub message: &'static str,
    #[serde(skip)]
    pub status: Option<rocket::http::Status>,
}
