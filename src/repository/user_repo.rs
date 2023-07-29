use crate::models::user_model::User;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::{InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

use std::env;
extern crate dotenv;
use dotenv::dotenv;

pub struct UserRepo {
    pub col: Collection<User>,
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

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            email: new_user.email.to_owned(),
            id: None,
            name: new_user.name.to_owned(),
            password: new_user.password.to_owned(),
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error create user");

        Ok(user)
    }

    pub fn get_user(&self, id: String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();

        let filter = doc! {"_id": obj_id};
        let user = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Failed to execute find_one.");

        Ok(user.unwrap())
    }

    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_user.id,
                "name": new_user.name,
                "email": new_user.email,
                "password": new_user.password,
            }
        };
        let update_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error, not is possible update this user");

        Ok(update_doc)
    }
}
