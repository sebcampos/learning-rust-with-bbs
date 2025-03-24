use std::io::Write;
use std::net::TcpStream;
use crate::db::manage::Manager;
use crate::input_interface::Events;
use crate::views::base_view::{NavigateTo, View};
use crate::views::base_view::NavigateTo::NoneView;

pub struct LoginRegisterView {
    username: String,
    user_id: i32,
    password: String,
    error: bool,
    error_message: Option<&'static str>,
    collecting_username: bool,
    collecting_password: bool,
    is_login: bool,
    is_create: bool,
    options: Vec<&'static str>,
    navigate_to: NavigateTo,
    selected_index: usize,
}


impl LoginRegisterView {
    pub fn new() -> LoginRegisterView {
        Self {
            user_id: -1,
            username: "".to_string(),
            password: "".to_string(),
            error: false,
            error_message: None,
            collecting_username: false,
            collecting_password: false,
            options: vec!["Login", "Register"],
            selected_index: 0,
            navigate_to:NoneView,
            is_login: false,
            is_create: false,
        }
    }

    fn login_register(&mut self) -> Events {
        let mut result_event: Events = Events::Unknown;
        let mut error_msg: &str = "";
        let user_id: i32;
        if self.is_login {
            user_id = Manager::validate_user(&self.username, &self.password);
            error_msg = "Unable to validate user, maybe wrong password?";
        }
        else if self.is_create {
            user_id =  Manager::create_user(&self.username, &self.password);
            error_msg = "Unable to create user, maybe username already taken";
        }
        else {
            user_id = -1
        }
        if user_id < 0 {
            self.error = true;
            self.error_message = Some(error_msg);
        }
        else {
            self.user_id = user_id;
            Manager::login_user(self.user_id);
            self.navigate_to = NavigateTo::MenuView;
            result_event = Events::Authenticate;
        }
        self.is_login = false;
        self.is_create = false;
        result_event
    }

}

impl View for LoginRegisterView {
    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn get_user_id(&self) -> i32 {
        self.user_id
    }

    fn render(&self) -> String {

        let mut output = String::from("\x1b[2J\x1b[H");
        if self.error {
            output.push_str("\x1b[1;31mLogin ERROR\x1b[0m\r\n\r\n");
            output.push_str(self.error_message.unwrap());
        }
        else {
            output.push_str("\x1b[1;32mLogin\x1b[0m\r\n\r\n");
        }

        if self.collecting_username && !self.error {
            output.push_str("\x1b[1;32m> Username: ");
        }
        else if self.collecting_password  && !self.error {
            output.push_str("\x1b[1;32m> Password: ");
        }
        else if !self.error {
            for (idx, option) in self.options.iter().enumerate() {
                if idx == self.selected_index {
                    output.push_str(&format!("\x1b[1;33m> {} \x1b[0m\r\n", option)); // Highlighted selection
                } else {
                    output.push_str(&format!("  {}\r\n", option));
                }
            }
            output.push_str("\nUse ↑ (Arrow Up) / ↓ (Arrow Down)");
        }
        output
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected_index < self.options.len() - 1 {
            self.selected_index += 1;
        }
    }

    fn get_selection(&mut self) -> &str {
        self.options[self.selected_index]
    }


    fn handle_selection(&mut self, stream: &mut TcpStream) -> Events {
        let selection = self.get_selection();
        let mut result_event: Events = Events::InputModeEnable;
        if selection == "Login" {
            self.is_login = true;
            self.collecting_username = true;
        } else if  selection == "Register" {
            self.is_create = true;
            self.collecting_username = true;
        }
        result_event
    }

    fn handle_event(&mut self, event: Events, stream: &mut TcpStream, buffer_string: Option<String>) -> Events {
        let result_event: Events;
        if self.error {
            result_event = Events::InputModeDisable;
            self.error = false;
        }
        else if event == Events::UpArrow {
            result_event = event;
            self.move_up();
        }
        else if event == Events::DownArrow {
            result_event = event;
            self.move_down();
        } else if event == Events::Enter {
            result_event = self.handle_selection(stream);
        } else if self.collecting_username  || self.collecting_password {
            let buffer_str = buffer_string.unwrap();
            if self.collecting_username && buffer_str.trim() != "" {
                self.username = buffer_str.trim().to_string();
                self.collecting_username = false;
                self.collecting_password = true;
                result_event = Events::SecretInputModeEnable;
            }
            else if self.collecting_password && buffer_str.trim() != "" {
                self.password = buffer_str.trim().to_string();
                self.collecting_password = false;
                result_event = self.login_register()
            }
            else {
                result_event = event;
            }
        } else {
            result_event = event
        }
        result_event
    }

}