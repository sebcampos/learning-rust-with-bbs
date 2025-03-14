use rusqlite::{params, Connection};
use crate::db::queries;

pub (crate) fn create_connection() -> Connection {
    // Connect to SQLite (creates 'bbs.db' if not exists)
    let conn = Connection::open("bbs.db").unwrap();

    // Create users table
    conn.execute(queries::CREATE_USERS, []).expect("Create failed");
    conn.execute(queries::CREATE_ROOMS, []).expect("Create rooms failed");
    conn.execute(queries::CREATE_ROOM_MESSAGES, []).expect("Create room messages failed");
    conn.execute(queries::CREATE_DIRECT_MESSAGES, []).expect("Create direct messages failed");

    println!("Database setup complete! ✅");
    conn
}
