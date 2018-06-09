extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use rocket::response::status::NotFound;
use rocket_contrib::Json;
use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

#[path = "pg.rs"]
mod pg;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub uname: String,
    pub email: String,
}

impl<'r> Responder<'r> for User {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!("{}:{}", self.id, self.uname)))
            .raw_header("X-User-Name", self.uname)
            // .raw_header("X-User-Age", self.age.to_string())
            .header(ContentType::JSON)
            .ok()
    }
}

impl User {
    pub fn add_user(user: Json<User>) -> Json<User> {
        pg::execute(
            &"INSERT INTO users(uname, email) VALUES ($1, $2)".to_string(),
            &[&user.uname, &user.email],
        );

        for row in &pg::query(&"SELECT id, uname, email FROM users".to_string(), &[]) {
            let user = User {
                id: row.get(0),
                uname: row.get(1),
                email: row.get(2),
            };
            println!("{}: {}: {}", user.id, user.uname, user.email);
        }

        user
    }
    pub fn get_user(id: i32) -> Result<Json<User>, NotFound<String>> {
        let rows = &pg::query(
            &"SELECT id, uname, email FROM users where id = $1 LIMIT 1".to_string(),
            &[&id],
        );

        if rows.is_empty() {
            return Result::Err(NotFound(format!("User not found")));
        }

        let row = &rows.iter().last().unwrap();

        Result::Ok(Json(User {
            id: row.get(0),
            uname: row.get(1),
            email: row.get(2),
        }))
    }
}
