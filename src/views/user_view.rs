use std::any::Any;
use std::collections::HashMap;
use std::net::TcpStream;
use crate::db::manage::Manager;
use crate::input_interface::Events;
use crate::views::base_view::{NavigateTo, View};

pub struct UserView{
    user_id: i32,
    navigate_to: NavigateTo,
    is_current_user: bool,
    user_data: HashMap<String, String>
}

impl UserView {
    pub fn new(user_id: i32, is_current_user: bool) -> Self {
        let user_data = Manager::get_user(user_id);
        Self {
            user_id,
            navigate_to: NavigateTo::NoneView,
            is_current_user,
            user_data
        }
    }

    fn move_up(&mut self) {}

    fn move_down(&mut self) {}

    fn get_selection(&mut self) -> &str {
        todo!()
    }

    fn get_user_id(&self) -> i32 {
        todo!()
    }

    fn handle_selection(&mut self, stream: &mut TcpStream) -> Events {
        todo!()
    }
}

impl View for UserView {


    fn as_any(&self) -> &(dyn Any) {
        self
    }

    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H");
        if self.is_current_user {
            output.push_str("\x1b[1;32mMe\x1b[0m\r\n\r\n");
        }
        else {
            output.push_str(&format!("\x1b[1;32m{}\x1b[0m\r\n\r\n", self.user_data["username"]));
        }


            // Append sorted rooms to output
        for (key, value) in self.user_data.iter() {
            if *key == "online" {
                let logged_in = value == "1";
                if logged_in {
                    output.push_str("🟢 online\r\n");
                }
                else {
                    output.push_str("⚪️  offline\r\n");
                }
            }
            else {
                output.push_str(&format!("{}: {}\r\n", key, value));
            }
        }
        if self.is_current_user {
            output.push_str("\n[H] Home\r\n");
        } else {
            output.push_str("\n[S] Send Message\r\n[H] Home\r\n");
        }
        output
    }


    fn handle_event(&mut self, event: Events, buffer_string: String) -> Events {
        let result_event: Events;

        if event == Events::KeyS {
            self.navigate_to = NavigateTo::DirectMessageView;
            result_event = Events::NavigateView
        } else if event == Events::KeyH {
            self.navigate_to = NavigateTo::MenuView;
            result_event = Events::NavigateView;

        } else {
            result_event = event;
        }
        result_event
    }

}