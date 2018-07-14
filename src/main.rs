#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

use rocket::response::status::NotFound;
use rocket::response::{content, NamedFile};
use rocket::Data;
use rocket_contrib::Json;
use std::fs;
use std::path::Path;
use uuid::Uuid;

mod pg;
mod user;
use user::User;

static UPLOAD_DIR: &'static str = "./uploads";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/user", format = "application/json", data = "<user>")]
fn add_user(user: Json<User>) -> Result<Json<User>, NotFound<content::Json<&'static str>>> {
    User::add_user(user)
}

#[get("/user/<id>")]
fn get_user(id: i32) -> Result<Json<User>, NotFound<content::Json<&'static str>>> {
    User::get_user(id)
}

#[post("/upload", format = "text/plain", data = "<data>")]
fn upload(data: Data) -> Result<String, String> {
    let filename = Uuid::new_v4().to_string();

    match data
        .stream_to_file(UPLOAD_DIR.to_owned() + "/" + &filename)
        .map(|n| n.to_string())
    {
        Result::Ok(_) => Ok("success".to_string()),
        Result::Err(err) => {
            if err.to_string() == "No such file or directory (os error 2)" {
                return match fs::create_dir_all(UPLOAD_DIR) {
                    Result::Ok(_) => Err("Please try again".to_string()),
                    Result::Err(_) => Err("Unknown error".to_string()),
                };
            }
            Err("Unknown error".to_string())
        }
    }
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

    match fs::create_dir_all(UPLOAD_DIR) {
        Result::Ok(_) => println!("Directory added: '{}'", UPLOAD_DIR),
        Result::Err(err) => println!("{}", err),
    };

    rocket::ignite()
        .mount("/", routes![index, favicon, robots])
        .mount("/api", routes![add_user, get_user, upload])
        .launch();
}
