#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::path::Path;

mod pg;
use pg::Psql;
mod user;
use user::{from_id, User};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/user", format = "application/json", data = "<user>")]
fn add_user(user: Json<User>) -> Json<User> {
    Psql::execute(
        &"INSERT INTO users(uname, email) VALUES ($1, $2)".to_string(),
        &[&user.uname, &user.email],
    );

    for row in &Psql::query(&"SELECT id, uname, email FROM users".to_string(), &[]) {
        let user = User {
            id: row.get(0),
            uname: row.get(1),
            email: row.get(2),
        };
        println!("{}: {}: {}", user.id, user.uname, user.email);
    }

    from_id(user)
}

#[get("/user/<id>")]
fn get_user(id: i32) -> Json<User> {
    let rows = &Psql::query(
        &"SELECT id, uname, email FROM users where id = $1 LIMIT 1".to_string(),
        &[&id],
    );

    let row = &rows.iter().last().unwrap();

    from_id(Json(User {
        id: row.get(0),
        uname: row.get(1),
        email: row.get(2),
    }))
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
    Psql::new();

    rocket::ignite()
        .mount("/", routes![index, favicon, robots])
        .mount("/api", routes![add_user, get_user])
        .launch();
}
