use rocket::{
    response::status::Custom,
    serde::json::{Json, Value},
};
use todo_backend::ResponseError;

use crate::{
    middleware::user::UserOnly,
    modules::todo::{
        create_todo::CreateTodoRepo, get_todo_by_category::ResponseTodoByCategory,
        get_todo_by_id::GetTodoByIdResponse,
    },
};

#[post("/", data = "<todo>")]
pub fn create_todo_router(
    todo: Json<CreateTodoRepo>,
    _user: UserOnly,
) -> Result<Custom<Json<Value>>, Custom<Json<ResponseError>>> {
    crate::modules::todo::create_todo::create_todo(todo)
}

#[get("/category/<id>")]
pub fn get_todo_by_category_router(
    id: String,
    user: UserOnly,
) -> Result<Json<Vec<ResponseTodoByCategory>>, Custom<Json<ResponseError>>> {
    crate::modules::todo::get_todo_by_category::get_todo_by_category(id, user)
}

#[get("/<id>")]
pub fn get_todo_by_id_router(
    id: String,
) -> Result<Json<GetTodoByIdResponse>, Custom<Json<ResponseError>>> {
    crate::modules::todo::get_todo_by_id::get_todo_by_id(id)
}

#[put("/<id>", data = "<data>")]
pub fn update_todo_router(
    id: String,
    data: Json<crate::modules::todo::update_todo::UpdateTodoRequest>,
) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    crate::modules::todo::update_todo::update_todo(id, data)
}

#[delete("/<id>")]
pub fn delete_todo_router(id: String) -> Result<Json<Value>, Custom<Json<ResponseError>>> {
    crate::modules::todo::delete_todo::delete_todo(id)
}

pub fn todo_routers() -> Vec<rocket::Route> {
    routes![
        create_todo_router,
        get_todo_by_category_router,
        get_todo_by_id_router,
        update_todo_router,
        delete_todo_router
    ]
}
