#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::path::Path;

mod user;
use user::{from_id, User};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user/<id>")]
fn user(id: usize) -> Json<User> {
    from_id(id)
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
    rocket::ignite()
        .mount("/", routes![index, favicon, robots])
        .mount("/api", routes![user])
        .launch();
}
