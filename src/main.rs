#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::path::Path;

mod pg;
mod user;
use user::User;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/user", format = "application/json", data = "<user>")]
fn add_user(user: Json<User>) -> Json<User> {
    User::add_user(user)
}

#[get("/user/<id>")]
fn get_user(id: i32) -> Result<Json<User>, NotFound<String>> {
    User::get_user(id)
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("./pub/favicon.ico")).ok()
}

#[get("/robots.txt")]
fn robots() -> Option<NamedFile> {
    NamedFile::open(Path::new("./pub/robots.txt")).ok()
}

fn main() {
    pg::new();

    rocket::ignite()
        .mount("/", routes![index, favicon, robots])
        .mount("/api", routes![add_user, get_user])
        .launch();
}
