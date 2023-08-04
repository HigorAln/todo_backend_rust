use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Json, Value},
};
use todo_backend::ResponseError;

use crate::repository::user::{update_user::UpdateUserParams, user_repo::UserRepo};

pub fn update_user(
    new_user: Json<UpdateUserParams>,
    user_id: &String,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    let collection = UserRepo::init();
    let update_result = collection.update_user(&user_id, new_user.into_inner());

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let update_user_info = collection.get_user(user_id.to_owned());

                return match update_user_info {
                    Ok(user) => {
                        let id = user.id.map(|id| id.to_hex()).unwrap();

                        Ok(Json(json!({
                            "id": id,
                            "name": user.name,
                            "email": user.email,
                        })))
                    }
                    Err(_) => Err(Custom(
                        Status::InternalServerError,
                        Json(ResponseError {
                            message: "Error, not is possible update this user",
                            status: None,
                        }),
                    )),
                };
            } else {
                return Err(Custom(
                    Status::NotFound,
                    Json(ResponseError {
                        message: "User not found",
                        status: None,
                    }),
                ));
            }
        }
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json(ResponseError {
                message: "Error, not is possible update this user",
                status: None,
            }),
        )),
    }
}
