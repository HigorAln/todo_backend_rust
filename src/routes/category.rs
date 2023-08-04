use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
};
use todo_backend::ResponseError;

use crate::{
    middleware::user::UserOnly,
    modules::category::{
        create_category::CreateCategoryRequest, update_category::UpdateCategoryRequest,
    },
};

#[get("/<id>")]
pub fn get_category_router(
    id: String,
    user: UserOnly,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    crate::modules::category::get_category::get_category(user, id)
}

#[post("/", data = "<category>")]
pub fn create_category_router(
    category: Json<CreateCategoryRequest>,
    user: UserOnly,
) -> Result<Custom<Json<Value>>, Custom<Json<ResponseError>>> {
    crate::modules::category::create_category::create_category(user.id, category)
}

#[put("/<id>", data = "<data>")]
pub fn update_category_router(
    user: UserOnly,
    id: String,
    data: Json<UpdateCategoryRequest>,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    crate::modules::category::update_category::update_category(user, id, data)
}

#[delete("/<id>")]
pub fn delete_category_router(
    user: UserOnly,
    id: String,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    crate::modules::category::delete_category::delete_category(id, user)
}

pub fn category_routers() -> Vec<rocket::Route> {
    routes![
        get_category_router,
        create_category_router,
        update_category_router,
        delete_category_router
    ]
}

pub fn verify_if_user_is_owner(user: UserOnly, id: &String) -> Result<(), ResponseError> {
    if user.id != *id {
        return Err(ResponseError {
            status: Some(Status::Unauthorized),
            message: "You are not the owner of this category",
        });
    }
    Ok(())
}
