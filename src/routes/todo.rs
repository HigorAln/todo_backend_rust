use mongodb::results::InsertOneResult;
use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::modules::todo::{
    create_todo::CreateTodoRepo, get_todo_by_category::ResponseTodoByCategory,
    get_todo_by_id::GetTodoByIdResponse,
};

#[post("/", data = "<todo>")]
pub fn create_todo_router(
    todo: Json<CreateTodoRepo>,
) -> Result<Json<InsertOneResult>, Custom<Json<ResponseError>>> {
    crate::modules::todo::create_todo::create_todo(todo)
}

#[get("/category/<id>")]
pub fn get_todo_by_category_router(
    id: String,
) -> Result<Json<Vec<ResponseTodoByCategory>>, Custom<Json<ResponseError>>> {
    crate::modules::todo::get_todo_by_category::get_todo_by_category(id)
}

#[get("/<id>")]
pub fn get_todo_by_id_router(
    id: String,
) -> Result<Json<GetTodoByIdResponse>, Custom<Json<ResponseError>>> {
    crate::modules::todo::get_todo_by_id::get_todo_by_id(id)
}

pub fn todo_routers() -> Vec<rocket::Route> {
    routes![
        create_todo_router,
        get_todo_by_category_router,
        get_todo_by_id_router
    ]
}
