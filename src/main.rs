#[macro_use]
extern crate rocket;

mod api;
mod models;
mod repository;

use api::user_api::create_user;
use repository::mongodb_repo::MongoRepo;

#[get("/")]
fn hello() -> &'static str {
    "Application is Running!"
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![hello, create_user])
}
