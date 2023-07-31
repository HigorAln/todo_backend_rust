use rocket::request::{FromRequest, Outcome, Request};

use crate::models::jwt_model::Jwt;

pub struct UserOnly {
    pub id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserOnly {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = match request.headers().get_one("Authorization") {
            Some(v) => v.to_string(),
            None => {
                return rocket::outcome::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
            }
        };

        let authorization = authorization.replace("Bearer ", "");
        let jwt_model = Jwt::init();

        dbg!(&authorization);

        match jwt_model.decode(&authorization) {
            Ok(v) => Outcome::Success(UserOnly {
                id: String::from(v),
            }),
            Err(_) => {
                return rocket::outcome::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
            }
        }
    }
}
