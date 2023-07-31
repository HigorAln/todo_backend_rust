use rocket::{
    request::{self, FromRequest},
    Request,
};

use crate::{
    models::user_model::{Role, User},
    repository::user::user_repo::UserRepo,
    shared::jwt::Jwt,
};

#[derive(Debug)]
pub struct AdminOnly {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminOnly {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let authorization = match request.headers().get_one("Authorization") {
            Some(v) => v.to_string(),
            None => {
                return rocket::outcome::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
            }
        };

        let authorization = authorization.replace("Bearer ", "");
        let jwt_model = Jwt::init();

        let authorization: String = match jwt_model.decode(&authorization) {
            Ok(v) => v,
            Err(_) => {
                return rocket::outcome::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
            }
        };

        let collection = UserRepo::init();
        let find_user = collection.get_user(authorization);
        let user = match find_user {
            Ok(user) => user,
            Err(_) => {
                return rocket::outcome::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
            }
        };

        match user.role.unwrap() {
            Role::Admin => rocket::outcome::Outcome::Success(AdminOnly {
                user: User {
                    email: user.email,
                    name: user.name,
                    id: user.id,
                    password: "".to_string(),
                    role: Some(Role::Admin),
                },
            }),
            _ => {
                return rocket::outcome::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
            }
        }
    }
}
