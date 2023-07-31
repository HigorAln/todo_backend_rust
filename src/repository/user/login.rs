use rocket::http::Status;
use todo_backend::ResponseError;

use crate::{models::user_model::User, shared::jwt::Jwt};
use mongodb::bson::doc;

use super::user_repo::UserRepo;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

impl UserRepo {
    pub fn login_user(
        &self,
        email: &String,
        password: &String,
    ) -> Result<LoginResponse, ResponseError> {
        dbg!(password, email);
        let filter = doc! {"email": email, "password": password};
        let user = self.col.find_one(filter, None);

        match user {
            Ok(user) => match user {
                Some(user) => {
                    let jwt_model = Jwt::init();
                    let token = jwt_model.encode(&user.id.unwrap().to_string());

                    let token_result = match token {
                        Ok(token) => token,
                        Err(_) => {
                            return Err(ResponseError {
                                message: "Error, not is possible create this token",
                                status: Some(Status::InternalServerError),
                            })
                        }
                    };

                    Ok(LoginResponse {
                        user,
                        token: token_result,
                    })
                }
                None => Err(ResponseError {
                    message: "Error, email or password is incorrect",
                    status: Some(Status::BadRequest),
                }),
            },
            Err(_) => Err(ResponseError {
                message: "Error, email or password is incorrect",
                status: Some(Status::BadRequest),
            }),
        }
    }
}
