use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{
    middleware::user::UserOnly, models::user_model::User, repository::user_repo::UserRepo,
};

#[get("/my-profile")]
pub fn get_my_profile(user: UserOnly) -> Result<Json<User>, Custom<Json<ResponseError>>> {
    let collection = UserRepo::init();
    let find_user = collection.get_user(user.id);

    match find_user {
        Ok(user) => Ok(Json(User {
            email: user.email,
            name: user.name,
            id: None,
            password: "".to_string(),
            role: user.role,
        })),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
