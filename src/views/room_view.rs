use std::net::TcpStream;
use crate::db::manage::Manager;
use crate::input_interface::Events;
use crate::views::base_view::{NavigateTo, View};


pub struct RoomView{
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
        let mut messages = Manager::get_message_from_room(room_id);
        messages.reverse();
        Self {
            user_id,
            room_id,
            navigate_to: NavigateTo::NoneView,
            messages,
            room_name,
            sending_message: false,
            message: String::new()
        }
    }


}

impl View for RoomView {
    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H");
        output.push_str(&format!("\x1b[1;32m{}\x1b[0m\r\n\r\n", self.room_name));


        // Append sorted rooms to output
        for (user_id, created_date, user, message) in self.messages.iter() {
            if *user_id == self.user_id {
                output.push_str(&format!("\x1b[1;32m[{}]\x1b[0m \x1b[38;5;214m{}\x1b[0m  {}\r\n", created_date, user, message));
            }
            else {
                output.push_str(&format!("\x1b[1;32m[{}]\x1b[0m \x1b[1;35m{}\x1b[0m  {}\r\n", created_date, user, message));
            }
        }
        output.push_str("\n");
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

    fn refresh_data(&mut self) {
        self.messages = Manager::get_message_from_room(self.room_id);
        self.messages.reverse();
    }

    fn handle_selection(&mut self, stream: &mut TcpStream) -> Events {
        todo!()
    }

    fn handle_event(&mut self, event: Events, stream: &mut TcpStream, buffer_string: Option<String>) -> Events {
        let result_event: Events;
        // TODO handle delete
        // if event == Events::KeyR && !self.sending_message {
        //     self.navigate_to = NavigateTo::RoomsView;
        //     result_event = Events::NavigateView;
        // } else if event == Events::KeyS && !self.sending_message {
        //     self.sending_message = true;
        //     result_event = event;
        //} else
        if event == Events::Enter {
            if self.message != "" {
                Manager::post_message(self.room_id, self.message.clone(), self.get_user_id());
                result_event = Events::RoomMessageSent;
                self.message.clear();
            }
            else {
                result_event = Events::Unknown;
            }
        } else if event != Events::Enter {
            let buffer_str = buffer_string.unwrap();
            if buffer_str.trim() != "" {
                self.message.push_str(buffer_str.as_str());
            }
            result_event = event;
        } else if event == Events::CntrlQ {
            self.navigate_to = NavigateTo::RoomView;
            result_event = Events::NavigateView
        } else {
            result_event = event;
        }
        result_event
    }
}