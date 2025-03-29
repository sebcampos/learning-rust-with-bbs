use std::any::Any;
use crate::db::manage::Manager;
use crate::input_interface::Events;
use crate::views::base_view::{NavigateTo, View};

pub struct DirectMessageView {
    query_offset: i32,
    user_id: i32,
    user_id_2: i32,
    navigate_to: NavigateTo,
    message: String,
    messages: Vec<(i32, String, String, String)>,
}


impl DirectMessageView {
    pub fn new(user_id: i32, user_id_2: i32) -> Self {
        let mut messages = Manager::get_direct_messages_for(user_id, user_id_2, 0);
        messages.reverse();
        Self {
            user_id,
            user_id_2,
            navigate_to: NavigateTo::NoneView,
            messages,
            message: String::new(),
            query_offset: 0
        }
    }

    /**
    * returns the recipient user id
    */
    pub fn to_user_id(&self) -> i32 {
        self.user_id_2
    }

}


impl View for DirectMessageView {

    fn as_any(&self) -> &(dyn Any) {
        self
    }

    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H");
        output.push_str("\x1b[1;32mMessages\x1b[0m\r\n\r\n");


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
        self.messages = Manager::get_direct_messages_for(self.user_id, self.user_id_2, self.query_offset);
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
                Manager::post_direct_message(self.user_id, self.user_id_2, self.message.clone());
                result_event = Events::DirectMessageSent;
                self.message.clear();
            }
            else {
                result_event = Events::Unknown;
            }
        } else if event == Events::CntrlQ {
            self.navigate_to = NavigateTo::MenuView;
            result_event = Events::NavigateView;

        } else if event != Events::Enter {
            self.message = buffer_string;
            result_event = event;
        } else {
            result_event = event;
        }
        result_event
    }

}