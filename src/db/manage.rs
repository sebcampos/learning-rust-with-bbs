use std::collections::HashMap;
use crate::db::connection::get_db_connection;
use crate::db::queries;
use bcrypt::{hash, DEFAULT_COST};
use bcrypt::{verify};

pub struct Manager;

pub struct User {

}


impl Manager {


    pub fn add_to_room_online(room_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::JOIN_ROOM).unwrap();
        stmt.execute([&room_id]).expect("Failed to add to room");
    }

    pub fn subtract_from_room_online(room_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::LEAVE_ROOM).unwrap();
        stmt.execute([&room_id]).expect("Failed to subtract from room");
    }

    pub fn login_user(user_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::LOGIN_USER).unwrap();
        stmt.execute([&user_id]).expect("Failed to login user");
    }

    pub fn logout_user(user_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::LOGOUT_USER).unwrap();
        stmt.execute([&user_id]).expect("Failed to logout user");
    }

    pub fn create_user(username: &String, password: &String) -> i32 {
        let password_hash = hash(password, DEFAULT_COST).expect("Failed to hash password");
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::CREATE_NEW_USER).unwrap();
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

    pub fn get_room_name_by_id(room_id: i32) -> String {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_ROOM_NAME).unwrap();

        // Execute the query and collect the rows into a HashMap
        let mut rows = stmt.query([&room_id]).unwrap();
        let mut room_name: String = "".to_string();
        while let Some(row) = rows.next().unwrap() {
            room_name = row.get("name").unwrap();
        }
        room_name

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

    pub fn get_user(user_id: i32)  -> HashMap<String, String> {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_USER).unwrap();
        let mut rows = stmt.query([&user_id]).unwrap();
        let mut user: HashMap<String, String> = HashMap::new();


        while let Some(row) = rows.next().unwrap() {

            let name: String = row.get("username").unwrap();
            let logged_in: i32 = row.get("logged_in").unwrap();
            let created_date: String = row.get("created_date").unwrap();

            // Insert the result into the HashMap, here id is the key and name is the value
            user.insert("username".to_string(), name);
            user.insert("online".to_string(), logged_in.to_string());
            user.insert("created_date".to_string(), created_date);
        }

        user
    }
    pub fn get_online_users() -> HashMap<String, bool> {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_ONLINE_USERS).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut users: HashMap<String, bool> = HashMap::new();


        while let Some(row) = rows.next().unwrap() {

            let name: String = row.get("username").unwrap();
            let logged_in: i32 = row.get("logged_in").unwrap();
            let online: bool;
            if logged_in == 1 {
                online = true;
            }
            else {
                online = false;
            }

            // Insert the result into the HashMap, here id is the key and name is the value
            users.insert(name, online);
        }

        users
    }

    pub fn get_room_id_by_name(room_name: String) -> i32{
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_ROOM_BY_NAME).unwrap();
        let mut rows = stmt.query([&room_name]).unwrap();

        if let Some(room) = rows.next().unwrap() {
            let id: i32 = room.get("id").unwrap();
            id
        } else {
            -1
        }
    }


    pub fn get_user_id_by_name(user_name: &str) -> i32{
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_USER_BY_NAME).unwrap();
        let mut rows = stmt.query([&user_name]).unwrap();

        if let Some(user) = rows.next().unwrap() {
            let id: i32 = user.get("id").unwrap();
            id
        } else {
            -1
        }
    }

    pub fn search_users(username_query: String) -> HashMap<String, bool> {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::SEARCH_USER).unwrap();
        let pattern = format!("%{}%", username_query);
        let mut rows = stmt.query([&pattern]).unwrap();

        // Create an empty HashMap to store the results
        let mut users: HashMap<String, bool> = HashMap::new();


        while let Some(row) = rows.next().unwrap() {

            let name: String = row.get("username").unwrap();
            let online: bool = row.get("logged_in").unwrap();

            // Insert the result into the HashMap, here id is the key and name is the value
            users.insert(name, online);
        }

        // Return the populated HashMap
        users
    }


    pub fn get_message_from_room(room_id: i32) -> Vec<(i32, String, String, String)> {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_MESSAGES_FOR_ROOM).unwrap();
        let mut rows = stmt.query([&room_id]).unwrap();

        // Create an empty HashMap to store the results
        let mut messages: Vec<(i32, String, String, String)> = Vec::new();
        while let Some(row) = rows.next().unwrap() {

            let user_id: i32 = row.get("user_id").unwrap();
            let name: String = row.get("username").unwrap();
            let message: String = row.get("message").unwrap();
            let created_date: String = row.get("created_date").unwrap();

            // Insert the result into the HashMap, here id is the key and name is the value
            messages.push((user_id, created_date, name, message));
        }
        messages
    }

    pub fn post_message(room_id: i32, message: String, user_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::POST_MESSAGE_TO_ROOM).unwrap();
        stmt.execute([&message, &user_id.to_string(), &room_id.to_string()]).expect("Failed to post to room");
    }

}