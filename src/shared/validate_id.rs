use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use todo_backend::ResponseError;

pub fn verify_object_id(id: &String) -> Result<ObjectId, ResponseError> {
    match ObjectId::parse_str(id) {
        Ok(obj) => return Ok(obj),
        Err(_) => {
            return Err(ResponseError {
                message: "This ID is not valid",
                status: Some(Status::BadRequest),
            })
        }
    }
}
