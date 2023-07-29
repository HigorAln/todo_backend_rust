#[macro_use]
extern crate rocket;

mod api;
mod models;
mod repository;

use api::user_api::{create_user, get_user, update_user};

#[get("/")]
fn hello() -> &'static str {
    "Application is Running!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, create_user, get_user, update_user])
}
