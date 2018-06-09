extern crate postgres;

use postgres::types::ToSql;
use postgres::{Connection, TlsMode};

pub struct Psql {}
static DSN: &'static str = "postgres://postgres:pass@localhost:5432";

impl Psql {
    pub fn new() {
        let conn = Connection::connect(DSN, TlsMode::None).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
            id   SERIAL PRIMARY KEY,
            uname VARCHAR NOT NULL,
            email VARCHAR NOT NULL)",
            &[],
        ).unwrap();
    }
    pub fn query(stmt: &String, args: &[&ToSql]) -> postgres::rows::Rows {
        let conn = Connection::connect(DSN, TlsMode::None).unwrap();

        conn.query(stmt, &args).unwrap()
    }

    pub fn execute(stmt: &String, args: &[&ToSql]) {
        let conn = Connection::connect(DSN, TlsMode::None).unwrap();

        conn.execute(stmt, &args).unwrap();
    }
}
