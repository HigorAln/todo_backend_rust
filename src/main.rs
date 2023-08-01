#[macro_use]
extern crate rocket;

mod middleware;
mod models;
mod modules;
mod repository;
mod shared;

use modules::{
    system_api::{health_check, login},
    todo::create_todo::create_todo,
    user::{
        create_user::create_user, delete_user::delete_user, get_user::get_user,
        my_profile::get_my_profile, update_user::update_user,
    },
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health_check, login])
        .mount(
            "/user",
            routes![
                create_user,
                get_user,
                update_user,
                delete_user,
                get_my_profile
            ],
        )
        .mount("/todo", routes![create_todo])
}
