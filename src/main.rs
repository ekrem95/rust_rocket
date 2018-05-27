#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate rust_rocket;
extern crate serde;
extern crate serde_json;

use rocket_contrib::Json;
use rust_rocket::{from_id, User};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user/<id>")]
fn user(id: usize) -> Json<User> {
    from_id(id)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api", routes![user])
        .launch();
}
