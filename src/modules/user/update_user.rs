use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{
    middleware::user::UserOnly, models::user_model::User, repository::user_repo::UserRepo,
};

#[put("/<id>", data = "<new_user>")]
pub fn update_user(
    id: String,
    new_user: Json<User>,
    _user: UserOnly,
) -> Result<Json<User>, Custom<Json<ResponseError>>> {
    let obj_id = match ObjectId::parse_str(&id) {
        Ok(obj) => obj,
        Err(_) => {
            return Err(Custom(
                Status::BadRequest,
                Json(ResponseError {
                    message: "This ID is not valid",
                    status: None,
                }),
            ))
        }
    };

    let data = User {
        id: Some(obj_id),
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        role: None,
    };

    let collection = UserRepo::init();
    let update_result = collection.update_user(&id, data);

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let update_user_info = collection.get_user(id);

                return match update_user_info {
                    Ok(user) => Ok(Json(user)),
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