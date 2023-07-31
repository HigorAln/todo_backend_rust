use crate::models::{jwt_model::Jwt, user_model::User};
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};
use rocket::http::Status;
use todo_backend::ResponseError;

use std::env;
extern crate dotenv;
use dotenv::dotenv;

pub struct UserRepo {
    pub col: Collection<User>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

impl UserRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Erro, variável nao encontrada"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rust_api");
        let col: Collection<User> = db.collection("user");

        UserRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, ResponseError> {
        let user = self.col.insert_one(new_user, None);

        match user {
            Ok(user) => Ok(user),
            Err(_) => Err(ResponseError {
                message: "Error, not is possible create this user",
                status: Some(Status::InternalServerError),
            }),
        }
    }

    pub fn get_user(&self, id: String) -> Result<User, ResponseError> {
        let obj_id: ObjectId = match verify_object_id(&id) {
            Ok(obj) => obj,
            Err(err) => return Err(err),
        };

        let not_fount_error = ResponseError {
            message: "user not found",
            status: Some(Status::NotFound),
        };

        let filter = doc! {"_id": obj_id};
        let user = self.col.find_one(filter, None);

        let result = match user {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => return Err(not_fount_error),
            },
            Err(_) => Err(not_fount_error),
        };

        match result {
            Ok(user) => Ok(User {
                email: user.email,
                name: user.name,
                id: user.id,
                password: None,
                role: user.role,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, ResponseError> {
        let obj_id: ObjectId = match verify_object_id(&id) {
            Ok(obj) => obj,
            Err(err) => return Err(err),
        };

        let filter = doc! { "_id": obj_id};

        let mut update_fields = doc! {};

        update_fields.insert("name", new_user.name);
        update_fields.insert("email", new_user.email);

        if let Some(password) = new_user.password {
            update_fields.insert("password", password);
        }

        let new_doc = doc! {
            "$set": update_fields
        };

        let update_doc = self.col.update_one(filter, new_doc, None);

        match update_doc {
            Ok(update_doc) => Ok(update_doc),
            Err(_) => Err(ResponseError {
                message: "Error, not is possible update this user",
                status: Some(Status::InternalServerError),
            }),
        }
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, ResponseError> {
        let obj_id: ObjectId = match verify_object_id(&id) {
            Ok(obj) => obj,
            Err(err) => return Err(err),
        };

        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.delete_one(filter, None);

        match user_detail {
            Ok(user_detail) => Ok(user_detail),
            Err(_) => Err(ResponseError {
                message: "Error, not is possible delete this user",
                status: Some(Status::InternalServerError),
            }),
        }
    }

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
                        user: User {
                            email: user.email,
                            name: user.name,
                            id: None,
                            password: None,
                            role: user.role,
                        },
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

fn verify_object_id(id: &String) -> Result<ObjectId, ResponseError> {
    match ObjectId::parse_str(id) {
        Ok(obj) => return Ok(obj),
        Err(_) => {
            return Err(ResponseError {
                message: "This ID is not valid",
                status: Some(Status::BadRequest),
            })
        }
    }
}
