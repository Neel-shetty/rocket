#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, from a rusty web server!. This is working"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
