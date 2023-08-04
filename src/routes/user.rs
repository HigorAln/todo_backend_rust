use mongodb::results::InsertOneResult;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
};
use todo_backend::ResponseError;

use crate::{
    middleware::{admin::AdminOnly, user::UserOnly},
    models::user_model::User,
    repository::user::update_user::UpdateUserParams,
};

#[get("/<id>")]
pub fn get_user_router(
    id: String,
    _admin: AdminOnly,
) -> Result<Json<User>, Custom<Json<ResponseError>>> {
    crate::modules::user::get_user::get_user(id)
}

#[delete("/<id>")]
pub fn delete_user_router(id: String, _admin: AdminOnly) -> Result<Status, Status> {
    crate::modules::user::delete_user::delete_user(id)
}

#[get("/my-profile")]
pub fn get_my_profile_router(user: UserOnly) -> Result<Json<User>, Custom<Json<ResponseError>>> {
    crate::modules::user::my_profile::get_my_profile(user)
}

#[put("/", data = "<new_user>")]
pub fn update_user_router(
    new_user: Json<UpdateUserParams>,
    user_id: UserOnly,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    crate::modules::user::update_user::update_user(new_user, &user_id.id)
}

#[post("/", data = "<new_user>")]
pub fn create_user_router(
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Custom<Json<ResponseError>>> {
    crate::modules::user::create_user::create_user(new_user)
}

pub fn user_routers() -> Vec<rocket::Route> {
    routes![
        get_user_router,
        delete_user_router,
        get_my_profile_router,
        update_user_router,
        create_user_router
    ]
}
