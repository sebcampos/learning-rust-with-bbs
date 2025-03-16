use std::collections::HashMap;
use rusqlite::Connection;
use crate::db::connection;
use crate::db::queries;

pub struct Manager {
    conn: Connection,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            conn:  connection::create_connection()
        }
    }

    fn close_connection(self) {
        self.conn.close().expect("Close failed");
    }

    pub fn get_rooms(&self) -> HashMap<String, u32> {
        // Run the query to get the rows
        let mut stmt = self.conn.prepare(queries::GET_ROOMS).unwrap();

        // Execute the query and collect the rows into a HashMap
        let mut rows = stmt.query([]).unwrap();

        // Create an empty HashMap to store the results
        let mut rooms: HashMap<String, u32> = HashMap::new();


        while let Some(row) = rows.next().unwrap() {

            let name: String = row.get("name").unwrap();
            let online: u32 = row.get("online").unwrap();

            // Insert the result into the HashMap, here id is the key and name is the value
            rooms.insert(name, online);
        }

        // Return the populated HashMap
        rooms
    }
}