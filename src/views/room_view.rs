use std::collections::HashMap;
use std::net::TcpStream;
use crate::db::manage::Manager;
use crate::input_interface::Events;
use crate::views::base_view::{NavigateTo, View};
use crate::views::user_view::UserView;

pub struct RoomView{
    user_id: i32,
    room_id: i32,
    room_name: String,
    navigate_to: NavigateTo,
    messages: Vec<(String, String, String)>,
    sending_message: bool
}

impl RoomView {
    pub fn new(room_id: i32, room_name: String, user_id: i32) -> Self {
        let messages = Manager::get_message_from_room(room_id);
        Self {
            user_id,
            room_id,
            navigate_to: NavigateTo::NoneView,
            messages,
            room_name,
            sending_message: false
        }
    }


}

impl View for RoomView {

    fn refresh_data(&mut self) {
        self.messages = Manager::get_message_from_room(self.room_id);
    }

    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H");
        output.push_str(&format!("\x1b[1;32m{}\x1b[0m\r\n\r\n", self.room_name));


        // Append sorted rooms to output
        for (created_date, user, message) in self.messages.iter() {
            output.push_str(&format!("\x1b[1;32m[{}]\x1b[0m \x1b[1;35m{}\x1b[0m  {}\r\n", created_date, user, message));
        }
        output.push_str("\n");
        if !self.sending_message {
            output.push_str("[S] Send Message\r\n[H] Home\r\n");
        }
        output
    }


    fn move_up(&mut self) {}

    fn move_down(&mut self) {}

    fn get_selection(&mut self) -> &str {
        todo!()
    }

    fn get_user_id(&self) -> i32 {
        self.user_id
    }

    fn handle_selection(&mut self, stream: &mut TcpStream) -> Events {
        todo!()
    }

    fn handle_event(&mut self, event: Events, stream: &mut TcpStream, buffer_string: Option<String>) -> Events {
        let result_event: Events;

        if event == Events::KeyH && !self.sending_message {
            self.navigate_to = NavigateTo::RoomsView;
            result_event = Events::NavigateView;
        } else if event == Events::KeyS && !self.sending_message {
            self.sending_message = true;
            result_event = Events::InputModeEnable;
        } else if event == Events::CntrlQ {
            self.sending_message = false;
            result_event = Events::InputModeDisable;
        } else if self.sending_message {
            let buffer_str = buffer_string.unwrap();
            if buffer_str.trim() != "" {
                Manager::post_message(self.room_id, buffer_str, self.get_user_id());
            }
            result_event = Events::RoomMessageSent;
        } else {
            result_event = event;
        }
        result_event
    }
}