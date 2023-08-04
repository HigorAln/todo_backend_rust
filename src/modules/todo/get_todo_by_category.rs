use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{
    middleware::user::UserOnly, repository::todo::todo_repo::TodoRepo,
    routes::category::verify_if_user_is_owner,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResponseTodoByCategory {
    title: String,
    description: Option<String>,
    priority: Option<u8>,
    done: bool,
    id: Option<String>,
}

pub fn get_todo_by_category(
    id: String,
    user: UserOnly,
) -> Result<Json<Vec<ResponseTodoByCategory>>, Custom<Json<ResponseError>>> {
    let _ = verify_if_user_is_owner(user, &id);
    let todo_repo = TodoRepo::init();

    let todos = todo_repo.get_todo_by_owner(id);

    match todos {
        Ok(v) => {
            let mapped_result = v
                .iter()
                .map(|todo| ResponseTodoByCategory {
                    title: todo.title.clone(),
                    description: todo.description.clone(),
                    priority: todo.priority,
                    done: todo.done,
                    id: match todo.id.clone() {
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
