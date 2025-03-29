use std::collections::HashMap;
use bcrypt::{hash, DEFAULT_COST};
use bcrypt::{verify};
use crate::db::connection::get_db_connection;
use crate::db::queries;


pub struct Manager;


impl Manager {

    /**
    * This method takes a room id and increments its online count by 1
    */
    pub fn add_to_room_online(room_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::JOIN_ROOM).unwrap();
        stmt.execute([&room_id]).expect("Failed to add to room");
    }

    /**
    * This method takes a room id and decrements its online count by 1
    */
    pub fn subtract_from_room_online(room_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::LEAVE_ROOM).unwrap();
        stmt.execute([&room_id]).expect("Failed to subtract from room");
    }


    /**
    * This method takes a user id and sets their online status to 1 / true
    */
    pub fn login_user(user_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::LOGIN_USER).unwrap();
        stmt.execute([&user_id]).expect("Failed to login user");
    }


    /**
    * This method takes a user id and sets their online status to 0 / false
    */
    pub fn logout_user(user_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::LOGOUT_USER).unwrap();
        stmt.execute([&user_id]).expect("Failed to logout user");
    }


    /**
    * This method takes a username and password and creates a new user
    */
    pub fn create_user(username: &str, password: &str) -> i32 {
        let binding = hash(password, DEFAULT_COST).expect("Failed to hash password");
        let password_hash = binding.as_str();
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

    /**
    * This method takes a username and password and validates the password is correct
    * for the provided username
    */
    pub fn validate_user(username: &str, password: &str) -> i32 {
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

    /**
    * Retrieves the room name for the provided room id
    */
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


    /**
    * gets the rooms ordered by active user count in descending order
    * offset can be used for pagination
    */
    pub fn get_rooms(offset: i32) -> Vec<(String, u32)> {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_ROOMS).unwrap();

        // Execute the query and collect the rows into a HashMap
        let mut rows = stmt.query([offset]).unwrap();

        // Create an empty HashMap to store the results
        let mut rooms: Vec<(String, u32)> = Vec::new();


        while let Some(row) = rows.next().unwrap() {

            let name: String = row.get("name").unwrap();
            let online: u32 = row.get("online").unwrap();

            // Insert the result into the HashMap, here id is the key and name is the value
            rooms.push((name, online));
        }

        // Return the populated HashMap
        rooms.reverse();
        rooms
    }

    /**
    * retrieves rooms with names matching the `room_query` param
    * offset can be used for pagination
    */
    pub fn search_rooms(room_query: String, offset: i32) ->  Vec<(String, u32)> {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::SEARCH_ROOMS).unwrap();
        let pattern = format!("%{}%", room_query);
        let mut rows = stmt.query([&pattern, &offset.to_string()]).unwrap();

        // Create an empty HashMap to store the results
        let mut rooms: Vec<(String, u32)> = Vec::new();


        while let Some(row) = rows.next().unwrap() {

            let name: String = row.get("name").unwrap();
            let online: u32 = row.get("online").unwrap();

            // Insert the result into the HashMap, here id is the key and name is the value
            rooms.push((name, online));
        }

        // Return the populated HashMap
        rooms

    }

    /**
    * creates a room using the provided `room_name` and `user_id`
    */
    pub fn create_room(room_name: String, user_id: String) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::CREATE_NEW_ROOM).unwrap();

        // Execute the query and collect the rows into a HashMap
        stmt.execute([&room_name, &user_id]).unwrap();
        println!("Created new room {}", room_name);
    }

    /**
    * creates the tables in the db if they do not already exist
    */
    pub fn setup_db() {
        let conn = get_db_connection().lock().unwrap();
        // Create users table
        conn.execute(queries::CREATE_USERS, []).expect("Create users failed");
        conn.execute(queries::CREATE_ROOMS, []).expect("Create rooms failed");
        conn.execute(queries::CREATE_ROOM_MESSAGES, []).expect("Create room messages failed");
        conn.execute(queries::CREATE_DIRECT_MESSAGES, []).expect("Create direct messages failed");

        println!("Database setup complete! ✅")

    }

    /**
    * Retrieve info on a user using the provided `user_id`
    */
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

    /**
    * retrieves users ordered by their logged in value (1 online, 0 offline)
    * `offset` can be used for pagination
    */
    pub fn get_online_users(offset: i32) ->  Vec<(String, bool)>  {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_ONLINE_USERS).unwrap();
        let mut rows = stmt.query([&offset]).unwrap();
        let mut users: Vec<(String, bool)>  = Vec::new();


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
            users.push((name, online));
        }

        users.reverse();
        users
    }

    /**
    * gets the room id for the provided `room_name`, -1 if no match found
    */
    pub fn get_room_id_by_name(room_name: String) -> i32 {
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

    /**
    * gets the user_id for the provided `user_name`, -1 if no match found
    */
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

    /**
    * searches for a user by username
    */
    pub fn search_users(username_query: String) -> Vec<(String, bool)>  {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::QUERY_BY_USERNAME).unwrap();
        let pattern = format!("%{}%", username_query);
        let mut rows = stmt.query([&pattern]).unwrap();

        // Create an empty HashMap to store the results
        let mut users: Vec<(String, bool)>  = Vec::new();


        while let Some(row) = rows.next().unwrap() {

            let name: String = row.get("username").unwrap();
            let online: bool = row.get("logged_in").unwrap();

            // Insert the result into the HashMap, here id is the key and name is the value
            users.push((name, online));
        }

        // Return the populated HashMap
        users
    }

    /**
    * collects the messages for a room ordered by created date in descending order
    * `offset` can be used for pagination
    */
    pub fn get_message_from_room(room_id: i32, offset: i32) -> Vec<(i32, String, String, String)> {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_MESSAGES_FOR_ROOM).unwrap();
        let mut rows = stmt.query([&room_id, &offset]).unwrap();

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

    /**
    * publishes a message to a room for the user
    */
    pub fn post_message(room_id: i32, message: String, user_id: i32) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::POST_MESSAGE_TO_ROOM).unwrap();
        stmt.execute([&message, &user_id.to_string(), &room_id.to_string()]).expect("Failed to post to room");
    }

    /**
    * saves a direct message between users
    */
    pub fn post_direct_message(user_id: i32, to_user_id: i32, message: String) {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::POST_DIRECT_MESSAGE).unwrap();
        stmt.execute([&message, &user_id.to_string(), &to_user_id.to_string()]).expect("Failed to post to room");
    }

    /**
    * retrieves the direct messages for between a set of users
    */
    pub fn get_direct_messages_for(user_id: i32, to_user_id: i32, offset: i32) -> Vec<(i32, String, String, String)> {
        let conn = get_db_connection().lock().unwrap();
        let mut stmt = conn.prepare(queries::GET_MESSAGES_FOR_USER).unwrap();
        let mut rows = stmt.query([&user_id, &to_user_id, &to_user_id, &user_id, &offset]).unwrap();

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
}