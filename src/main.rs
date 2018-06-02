#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use postgres::{Connection, TlsMode};
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
    let conn =
        Connection::connect("postgres://postgres:pass@localhost:5432", TlsMode::None).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id   SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        data BYTEA
    )",
        &[],
    ).unwrap();

    let me = User {
        id: 0,
        name: "Ekrem".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO users(name, data) VALUES ($1, $2)",
        &[&me.name, &me.data],
    ).unwrap();

    for row in &conn.query("SELECT id, name, data FROM users", &[]).unwrap() {
        let user = User {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("{}: {}", user.id, user.name);
    }

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
