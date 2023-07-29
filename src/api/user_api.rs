use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json};

use crate::{models::user_model::User, repository::user_repo::UserRepo};

#[post("/user", data = "<new_user>")]
pub fn create_user(new_user: Json<User>) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };

    let collection = UserRepo::init();
    let user = collection.create_user(data);

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<id>")]
pub fn get_user(id: String) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    }

    let collection = UserRepo::init();
    let find_user = collection.get_user(id);

    match find_user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/user/<id>", data = "<new_user>")]
pub fn update_user(id: String, new_user: Json<User>) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };

    let collection = UserRepo::init();
    let update_result = collection.update_user(&id, data);

    dbg!(&update_result);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let update_user_info = collection.get_user(id);

                return match update_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
