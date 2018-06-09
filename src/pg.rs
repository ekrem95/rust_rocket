extern crate postgres;

use postgres::types::ToSql;
use postgres::{Connection, TlsMode};

static DSN: &'static str = "postgres://postgres:pass@localhost:5432";

#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn query(stmt: &String, args: &[&ToSql]) -> postgres::rows::Rows {
    let conn = Connection::connect(DSN, TlsMode::None).unwrap();

    conn.query(stmt, &args).unwrap()
}
#[allow(dead_code)]
pub fn execute(stmt: &String, args: &[&ToSql]) {
    let conn = Connection::connect(DSN, TlsMode::None).unwrap();

    conn.execute(stmt, &args).unwrap();
}
