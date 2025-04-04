use std::any::Any;
use crate::db::manage::Manager;
use crate::input_interface::Events;
use crate::views::base_view::{NavigateTo, View};


pub struct RoomView{
    query_offset: i32,
    user_id: i32,
    room_id: i32,
    room_name: String,
    navigate_to: NavigateTo,
    message: String,
    messages: Vec<(i32, String, String, String)>,
    sending_message: bool
}

impl RoomView {
    pub fn new(room_id: i32, room_name: String, user_id: i32) -> Self {
        let mut messages = Manager::get_message_from_room(room_id, 0);
        messages.reverse();
        Self {
            user_id,
            room_id,
            navigate_to: NavigateTo::NoneView,
            messages,
            room_name,
            sending_message: false,
            message: String::new(),
            query_offset: 0
        }
    }

    fn get_user_id(&self) -> i32 {
        self.user_id
    }



}

impl View for RoomView {

    fn as_any(&self) -> &(dyn Any) {
        self
    }
    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H");
        output.push_str(&format!("\x1b[1;32m{}\x1b[0m\r\n\r\n", self.room_name));


        // Append sorted rooms to output
        for (user_id, created_date, user, message) in self.messages.iter() {
            if *user_id == self.user_id {
                output.push_str(&format!("\x1b[1;32m[{}]\x1b[0m \x1b[1;35m{}\x1b[0m  {}\r\n", created_date, user, message));
            }
            else {
                output.push_str(&format!("\x1b[1;32m[{}]\x1b[0m \x1b[38;5;214m{}\x1b[0m  {}\r\n", created_date, user, message));
            }
        }
        output.push_str("\n\x1b[1;35m>>>\x1b[0m ");
        output.push_str(self.message.as_str());
        output
    }

    fn refresh_data(&mut self) {
        self.messages = Manager::get_message_from_room(self.room_id, self.query_offset);
        self.messages.reverse();
    }


    fn handle_event(&mut self, event: Events, buffer_string: String) -> Events {
        let mut result_event: Events = Events::Unknown;

        if event == Events::UpArrow  && self.messages.len() == 20 {
            self.query_offset += 1;
            self.refresh_data();
        }

        else if event == Events::DownArrow && self.query_offset != 0 {
            self.query_offset -= 1;
            self.refresh_data();
        }

        else if event == Events::Enter {
            if self.message != "" {
                Manager::post_message(self.room_id, self.message.clone(), self.get_user_id());
                result_event = Events::RoomMessageSent;
                self.message.clear();
            }
            else {
                result_event = Events::Unknown;
            }
        } else if event == Events::CntrlQ {
            self.navigate_to = NavigateTo::RoomsView;
            result_event = Events::RoomLeave
        } else if event != Events::Enter {
            self.message = buffer_string;
            result_event = event;
        } else {
            result_event = event;
        }
        result_event
    }
}