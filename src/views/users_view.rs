use std::collections::HashMap;
use std::net::TcpStream;
use crate::views::base_view::{NavigateTo, View};
use crate::input_interface::Events;
use crate::db::manage::Manager;


pub struct UsersView {
    users: HashMap<String, bool>,
    display_users: Vec<(String, bool)>,
    navigate_to: NavigateTo,
    user_id: i32,
    selecting_user: bool,
    searching_user: bool,
    selected_index: usize,
}

impl UsersView {

    pub fn new(user_id: i32) -> Self {
        let users = Manager::get_online_users();
        let display_users = users.iter().map(|(k, v)| (k.clone(), *v))
            .collect();
        let navigate_to: NavigateTo = NavigateTo::NoneView;
        let selecting_user = true;
        let searching_user = false;
        Self {
            users,
            display_users,
            navigate_to,
            selecting_user,
            searching_user,
            selected_index: 0,
            user_id
        }
    }


    fn refresh_users(&mut self, new_users: Option<HashMap<String, bool>>) {
        if let Some(new_users) = new_users {
            self.users = new_users;
        }
        else {
            self.users = Manager::get_online_users();
        }
        let mut vec: Vec<(String, bool)> = self.users.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        vec.sort_by(|a, b| b.1.cmp(&a.1));
        self.display_users = vec;
    }

}

impl View for UsersView {
    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }
    
    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H");
        output.push_str("\x1b[1;32mUsers\x1b[0m\r\n\r\n");


        if self.searching_user {
            output.push_str("\x1b[1;33m> Search (\"q\" to quit): ");
        } else if self.selecting_user {
            // Append sorted rooms to output
            for (index, (user, online)) in self.display_users.iter().enumerate() {
                let online_emoji;
                if *online {
                    online_emoji = "🟢 online"
                }
                else {
                    online_emoji = "⚪️ offline"
                }
                if index == self.selected_index {
                    output.push_str(&format!("\x1b[1;33m> {}: {}\x1b[0m\r\n", user, online_emoji));
                }
                else {
                    output.push_str(&format!("  {}: {}\r\n", user, online_emoji));
                }
            }
            output.push_str("\nUse ↑ (Arrow Up) / ↓ (Arrow Down) and Enter to select a user\r\n[S] Search for a user.\r\n[N] Next Page\r\n[H] Home\r\n");
        }
        output
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }


    fn move_down(&mut self) {
        let display_size: usize = self.display_users.len();
        if display_size > 0 && self.selected_index < display_size - 1 {
            self.selected_index += 1;
        }
    }

    fn get_selection(&mut self) -> &str {
        &self.display_users[self.selected_index].0
    }

    fn get_user_id(&self) -> i32 {
        self.user_id
    }


    fn handle_selection(&mut self, stream: &mut TcpStream) -> Events {
        Events::UserView
    }

    fn handle_event(&mut self, event: Events, stream: &mut TcpStream, buffer_string: Option<String>) -> Events {
        let result_event: Events;

        if event == Events::UpArrow {
            result_event = event;
            self.move_up();
        }
        else if event == Events::DownArrow {
            result_event = event;
            self.move_down();
        }
        else if event == Events::Enter {
            result_event = self.handle_selection(stream);
        }
        else if event == Events::KeyH && !self.searching_user
        {
            self.navigate_to = NavigateTo::MenuView;
            result_event = Events::NavigateView;
        }
        else if event == Events::KeyS && !self.searching_user
        {
            self.searching_user = true;
            result_event = Events::InputModeEnable;
        }

        else if self.searching_user {
            let buffer_str = buffer_string.unwrap();
            if buffer_str.trim() == "q" {
                self.searching_user = false;
                result_event = Events::InputModeDisable;
            } else if self.searching_user && buffer_str.trim() != "" {
                let user_query = Manager::search_users(buffer_str);
                self.refresh_users(Some(user_query));
                self.searching_user = false;
                result_event =  Events::InputModeDisable;;
            } else {
                result_event = event;
            }
        } else {
            result_event = event;
        }
        result_event
    }

}