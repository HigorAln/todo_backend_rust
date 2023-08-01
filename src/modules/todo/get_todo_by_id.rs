use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::repository::todo::todo_repo::TodoRepo;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetTodoByIdResponse {
    _id: String,
    title: String,
    description: Option<String>,
    priority: Option<u8>,
    done: bool,
    category: String,
}

#[get("/<id>")]
pub fn get_todo_by_id(
    id: String,
) -> Result<Json<GetTodoByIdResponse>, Custom<Json<ResponseError>>> {
    let todo_repo = TodoRepo::init();

    let todo = todo_repo.get_todo_by_id(&id);

    match todo {
        Ok(v) => Ok(Json(GetTodoByIdResponse {
            _id: v.id.unwrap().to_hex(),
            title: v.title,
            description: v.description,
            priority: v.priority,
            done: v.done,
            category: v.id.unwrap().to_hex(),
        })),
        Err(e) => Err(Custom(e.status.unwrap(), Json(e))),
    }
}
