use mongodb::results::InsertOneResult;
use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::repository::todo::{create_todo::CreateTodoRepo, todo_repo::TodoRepo};

#[post("/", data = "<todo>")]
pub fn create_todo(
    todo: Json<CreateTodoRepo>,
) -> Result<Json<InsertOneResult>, Custom<Json<ResponseError>>> {
    let collection = TodoRepo::init();
    let todo = collection.create_todo(todo.into_inner());

    match todo {
        Ok(todo) => Ok(Json(todo)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
