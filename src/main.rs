#[macro_use]
extern crate rocket;

mod middleware;
mod models;
mod modules;
mod repository;
mod routes;
mod shared;

use routes::{
    category::category_routers, system::system_routes, todo::todo_routers, user::user_routers,
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", system_routes())
        .mount("/user", user_routers())
        .mount("/todo", todo_routers())
        .mount("/category", category_routers())
}
