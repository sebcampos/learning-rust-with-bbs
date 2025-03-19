use rusqlite::{params, Connection};
use crate::db::{queries};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    Mutex::new(Connection::open("bbs.db").expect("Failed to open DB"))
});

pub (crate) fn get_db_connection() -> &'static Mutex<Connection> {
    &CONN
}

