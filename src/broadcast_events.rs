use serde::{Deserialize};
use serde_json::{Value};
use serde_json;
use crate::db::manage::Manager;
use crate::input_interface::UserInterface;

#[derive(Deserialize, Debug)]
struct UserEvent {
    user_id: i32,
    event_type: String
}

#[derive(Deserialize, Debug)]
struct UserDirectMessageEvent {
    sender_id: i32,
    receiver_id: i32,
    message: String
}


#[derive(Deserialize, Debug)]
struct RoomJoinEvent {
    user_id: i32,
    room_id: i32
}

#[derive(Deserialize, Debug)]
struct RoomLeaveEvent {
    user_id: i32,
    room_id: i32
}

#[derive(Deserialize, Debug)]
struct RoomExitEvent {
    user_id: i32,
    room_id: i32
}

#[derive(Deserialize, Debug)]
struct RoomMessageEvent {
    user_id: i32,
    room_id: i32,
    message: String,
}



pub fn handle_broadcast_event(raw_event: &str, ui: UserInterface) {
    // TODO maybe pass the UI instance in here
    let json: Value = serde_json::from_str(raw_event).unwrap();
    if json["event_type"] == "room_message" {

    }


}