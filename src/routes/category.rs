use mongodb::results::InsertOneResult;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{Json, Value},
};
use todo_backend::ResponseError;

use crate::{
    middleware::user::UserOnly, modules::category::create_category::CreateCategoryRequest,
};

#[get("/<id>")]
pub fn get_category_router(
    id: String,
    user: UserOnly,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    crate::modules::category::get_category::get_category(id)
}

#[post("/", data = "<category>")]
pub fn create_category_router(
    category: Json<CreateCategoryRequest>,
    user: UserOnly,
) -> Result<Json<InsertOneResult>, Custom<Json<ResponseError>>> {
    crate::modules::category::create_category::create_category(user.id, category)
}

pub fn category_routers() -> Vec<rocket::Route> {
    routes![get_category_router, create_category_router]
}
