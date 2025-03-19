use std::collections::HashMap;
use crate::db::connection::get_db_connection;
use crate::db::queries;
use rusqlite::{params};
use bcrypt::{hash, DEFAULT_COST};
use bcrypt::{verify};

pub struct Manager;



impl Manager {


    pub fn create_user(username: &String, password: &String) -> i32 {
        let password_hash = hash(password, DEFAULT_COST).expect("Failed to hash password");
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::CREATE_NEW_USER).unwrap();

        // Execute the query and collect the rows into a HashMap
        // let affected_rows = stmt.execute([&username, &password_hash]).unwrap();
        // if affected_rows >  0 {
        //     let user_id = conn.last_insert_rowid() as i32;
        //     return user_id;
        // }

        match stmt.execute([&username, &password_hash]) {
            Ok(affected_rows) => {
                if affected_rows > 0 {
                    let user_id = conn.last_insert_rowid() as i32;
                    user_id
                } else {
                    println!("No rows were inserted.");
                    -1 // Or handle this case properly
                }
            }
            Err(err) => {
                println!("Error inserting user: {:?}", err);
                -1 // Handle the error case (e.g., return a special value or propagate the error)
            }
        }
    }

    pub fn validate_user(username: &String, password: &String) -> i32 {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::SEARCH_USER).unwrap();
        let mut rows = stmt.query([&username]).unwrap();

        if let Some(user) = rows.next().unwrap() {
            let id: i32 = user.get("id").unwrap();
            let password_hash: String = user.get("password_hash").unwrap();
            let valid = verify(password, &password_hash).unwrap_or(false);
            if valid { id }  else { -1 }
        } else {
            -1
        }
    }

    pub fn get_rooms() -> HashMap<String, u32> {
        // Run the query to get the rows
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_ROOMS).unwrap();

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

    pub fn search_rooms(room_query: String) -> HashMap<String, u32> {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::SEARCH_ROOMS).unwrap();
        let pattern = format!("%{}%", room_query);
        let mut rows = stmt.query([&pattern]).unwrap();

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


    pub fn create_room(room_name: String, user_id: String) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::CREATE_NEW_ROOM).unwrap();

        // Execute the query and collect the rows into a HashMap
        let mut rows = stmt.execute([&room_name, &user_id]).unwrap();
        println!("Created new room {}", room_name);
    }

    pub fn setup_db() {
        let conn = get_db_connection().lock().unwrap();
        // Create users table
        conn.execute(queries::CREATE_USERS, []).expect("Create users failed");
        conn.execute(queries::CREATE_ROOMS, []).expect("Create rooms failed");
        conn.execute(queries::CREATE_ROOM_MESSAGES, []).expect("Create room messages failed");
        conn.execute(queries::CREATE_DIRECT_MESSAGES, []).expect("Create direct messages failed");

        println!("Database setup complete! ✅")

    }

}