extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub data: Option<Vec<u8>>,
}

impl<'r> Responder<'r> for User {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}:{}", self.id, self.name)))
            .raw_header("X-User-Name", self.name)
            // .raw_header("X-User-Age", self.age.to_string())
            .header(ContentType::JSON)
            .ok()
    }
}

pub fn from_id(_: usize) -> Json<User> {
    Json(User {
        id: 0,
        name: "eko".to_string(),
        data: None,
    })
}
