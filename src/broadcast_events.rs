use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
// use serde::{Deserialize};
use serde_json::{Value};
use serde_json;

use crate::input_interface::UserInterface;

// #[derive(Deserialize, Debug)]
// struct UserEvent {
//     user_id: i32,
//     event_type: String
// }
//
// #[derive(Deserialize, Debug)]
// struct UserDirectMessageEvent {
//     sender_id: i32,
//     receiver_id: i32,
//     message: String
// }
//
//
// #[derive(Deserialize, Debug)]
// struct RoomJoinEvent {
//     user_id: i32,
//     room_id: i32
// }
//
// #[derive(Deserialize, Debug)]
// struct RoomLeaveEvent {
//     user_id: i32,
//     room_id: i32
// }
//
// #[derive(Deserialize, Debug)]
// struct RoomExitEvent {
//     user_id: i32,
//     room_id: i32
// }
//
// #[derive(Deserialize, Debug)]
// struct RoomMessageEvent {
//     user_id: i32,
//     room_id: i32,
//     message: String,
// }



pub fn handle_broadcast_event(raw_event: String, ui: &Arc<Mutex<UserInterface>>, s_ref: &Arc<Mutex<TcpStream>>) -> i32 {

    let json: Value = serde_json::from_str(&*raw_event).unwrap_or(Value::Null);
    if json.is_null() {
        return 0;
    }
    let mut interface = ui.lock().unwrap();

    if json["event_type"] == "room_message" || json["event_type"] == "room_leave"  {
        let mut stream = s_ref.lock().unwrap();
        if json["room_id"] == interface.get_current_room_id() {
            let binding = interface.get_current_view();
            let mut view = binding.lock().unwrap();
            view.refresh_data();
            let updated_view = view.render();
            stream.write_all(updated_view.as_bytes()).unwrap();
            stream.flush().unwrap();

        }
    }
    else if json["event_type"] == "logout" && json["user_id"] == interface.get_user_id() {
        return -1;
    }


    else if json["event_type"] == "direct_message" && (json["user_id"] == interface.get_user_id() || json["to_user_id"] == interface.get_user_id()) {
        let mut stream = s_ref.lock().unwrap();
        let binding = interface.get_current_view();
        let mut view = binding.lock().unwrap();
        view.refresh_data();
        let updated_view = view.render();
        println!("TcpStream is locked and ready to write.");
        stream.write_all(updated_view.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    0


}