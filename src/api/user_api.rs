use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{
    middleware::{admin::AdminOnly, user::UserOnly},
    models::user_model::{Role, User},
    repository::user_repo::UserRepo,
};

#[post("/", data = "<new_user>")]
pub fn create_user(
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Custom<Json<ResponseError>>> {
    let data: User = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        role: Some(Role::User),
    };

    let collection = UserRepo::init();
    let user = collection.create_user(data);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}

#[get("/<id>")]
pub fn get_user(id: String, _admin: AdminOnly) -> Result<Json<User>, Custom<Json<ResponseError>>> {
    let collection = UserRepo::init();
    let find_user = collection.get_user(id);

    match find_user {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}

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

#[delete("/<id>")]
pub fn delete_user(id: String, _admin: AdminOnly) -> Result<Status, Status> {
    let collection = UserRepo::init();
    let delete_result = collection.delete_user(&id);

    match delete_result {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/my-profile")]
pub fn get_my_profile(user: UserOnly) -> Result<Json<User>, Custom<Json<ResponseError>>> {
    let collection = UserRepo::init();
    let find_user = collection.get_user(user.id);

    match find_user {
        Ok(user) => Ok(Json(User {
            email: user.email,
            name: user.name,
            id: None,
            password: None,
            role: user.role,
        })),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
