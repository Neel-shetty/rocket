#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

mod routes;
mod services;

use routes::index::index;
use routes::users::{create_role, create_user, get_users, update_user};

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(500)]
fn server_error() -> Value {
    json!({
        "status": "Server error",
        "reason": "Something went wrong. Please try again later"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, get_users, create_role, create_user, update_user],
        )
        .register("/", catchers![not_found, server_error])
}

// #[macro_use]
// extern crate rocket;
//
// #[get("/")]
// fn index() -> &'static str {
//     "Hello, from a rusty web server!. This is working"
// }
//
// #[get("/hi")]
// fn hello() -> &'static str {
//     "Hehe boi"
// }
//
// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![hello])
// }
