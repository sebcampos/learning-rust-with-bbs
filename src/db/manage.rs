use rusqlite::Connection;
use crate::db::connection;
use crate::db::queries;

struct Manager {
    conn: Connection,
}

impl Manager {
    fn new() -> Self {
        Self {
            conn:  connection::create_connection()
        }
    }

    fn close_connection(self) {
        self.conn.close().expect("Close failed");
    }

    fn get_rooms(self) -> rusqlite::Result<usize>{
        self.conn.execute(queries::GET_ROOMS, [])
    }
}