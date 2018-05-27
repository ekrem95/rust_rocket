#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u16,
}

impl<'r> Responder<'r> for User {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}:{}", self.name, self.age)))
            .raw_header("X-User-Name", self.name)
            .raw_header("X-User-Age", self.age.to_string())
            .header(ContentType::JSON)
            .ok()
    }
}

pub fn from_id(_: usize) -> Json<User> {
    Json(User {
        name: "eko".to_string(),
        age: 16,
    })
}
