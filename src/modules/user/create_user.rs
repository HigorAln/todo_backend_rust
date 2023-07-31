use mongodb::results::InsertOneResult;
use rocket::{response::status::Custom, serde::json::Json};
use todo_backend::ResponseError;

use crate::{
    models::user_model::{Role, User},
    repository::user::user_repo::UserRepo,
};

#[post("/", data = "<new_user>")]
pub fn create_user(
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Custom<Json<ResponseError>>> {
    let data: User = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        role: Some(Role::User),
    };

    let collection = UserRepo::init();
    let user = collection.create_user(data);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
    }
}
