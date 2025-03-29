use std::any::Any;
use crate::views::base_view::{NavigateTo, View};
use crate::input_interface::Events;
use crate::db::manage::Manager;


pub struct UsersView {
    users: Vec<(String, bool)>,
    navigate_to: NavigateTo,
    selecting_user: bool,
    searching_user: bool,
    selected_index: usize,
    query: String,
}

impl UsersView {

    pub fn new(user_id: i32) -> Self {
        let users = Manager::get_online_users(0);
        let navigate_to: NavigateTo = NavigateTo::NoneView;
        let selecting_user = true;
        let searching_user = false;
        Self {
            users,
            navigate_to,
            selecting_user,
            searching_user,
            selected_index: 0,
            query: String::new(),
        }
    }


    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }


    fn move_down(&mut self) {
        let display_size: usize = self.users.len();
        if display_size > 0 && self.selected_index < display_size - 1 {
            self.selected_index += 1;
        }
    }

    pub fn get_selection(&self) -> &str {
        &self.users[self.selected_index].0
    }



}

impl View for UsersView {


    fn as_any(&self) -> &(dyn Any) {
        self
    }


    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H");
        output.push_str("\x1b[1;32mUsers\x1b[0m\r\n\r\n");


        if self.searching_user {
            output.push_str("\x1b[1;33m> Search (CNTRL+Q to exit): ");
            output.push_str(self.query.as_str());
        } else if self.selecting_user {
            // Append sorted rooms to output
            for (index, (user, online)) in self.users.iter().enumerate() {
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
            output.push_str("\nUse ↑ (Arrow Up) / ↓ (Arrow Down) and Enter to select a user\r\n[S] Search for a user.\r\n[N] Next Page\r\n[H / CNTRL+Q] Home\r\n");
        }
        output
    }

    fn refresh_data(&mut self) {
        if !self.query.is_empty() {
            self.users = Manager::search_users(self.query.clone());
        }

        else {
            self.users = Manager::get_online_users(0);
        }
    }



    fn handle_event(&mut self, event: Events, buffer_string: String) -> Events {
        let mut result_event: Events = Events::Unknown;

        if event == Events::UpArrow && !self.searching_user {
            self.move_up();
        }
        else if event == Events::DownArrow && !self.searching_user {
            self.move_down();
        }
        else if event == Events::Enter && !self.searching_user {
            self.navigate_to = NavigateTo::UserView;
            result_event = Events::NavigateView;
        }
        else if event == Events::KeyH && !self.searching_user
        {
            self.navigate_to = NavigateTo::MenuView;
            result_event = Events::NavigateView;
        }
        else if event == Events::KeyS && !self.searching_user
        {
            self.searching_user = true;
            self.selecting_user = false;
            result_event = Events::NavigateView;

        }

        else if event == Events::CntrlQ && !self.searching_user {
            self.navigate_to = NavigateTo::MenuView;
            result_event = Events::NavigateView
        }

        else if event == Events::CntrlQ && self.searching_user {
            self.selecting_user = true;
            self.searching_user = false;
            self.query = String::new();
            result_event = Events::InputModeDisable
        }


        else if self.searching_user && buffer_string != ""  && event != Events::Enter {
             self.query = buffer_string;
        }

        else if self.searching_user && self.query != ""  && event == Events::Enter {
            self.refresh_data();
            self.query = String::new();
            self.searching_user = false;
            self.selecting_user = true;
            result_event = Events::InputModeDisable;
        }

        if result_event != Events::Unknown {
            result_event
        }
        else {
            event
        }
    }

}