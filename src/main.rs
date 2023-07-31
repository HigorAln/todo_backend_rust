#[macro_use]
extern crate rocket;

mod api;
mod middleware;
mod models;
mod repository;

use api::{
    system_api::{health_check, login},
    user_api::{create_user, delete_user, get_my_profile, get_user, update_user},
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
}
