use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{models::todo_model::Todo, repository::todo::todo_repo::TodoRepo};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResponseTodoByCategory {
    title: String,
    description: Option<String>,
    priority: Option<u8>,
    done: bool,
    _id: Option<String>,
}

pub fn get_todo_by_category(
    id: String,
) -> Result<Json<Vec<ResponseTodoByCategory>>, Custom<Json<ResponseError>>> {
    let todo_repo = TodoRepo::init();

    let todos = todo_repo.get_todo_by_owner(id);

    match todos {
        Ok(v) => {
            let todos: Vec<Todo> = v.map(|todo| todo.unwrap()).collect();

            let mapped_result = todos
                .iter()
                .map(|todo| ResponseTodoByCategory {
                    title: todo.title.clone(),
                    description: todo.description.clone(),
                    priority: todo.priority,
                    done: todo.done,
                    _id: match todo.id.clone() {
                        Some(v) => Some(v.to_hex()),
                        None => None,
                    },
                })
                .collect();

            Ok(Json(mapped_result))
        }
        Err(e) => Err(Custom(e.status.unwrap(), Json(e))),
    }
}
