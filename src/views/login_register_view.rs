use std::any::Any;
use std::io::Write;
use std::net::TcpStream;
use crate::db::manage::Manager;
use crate::input_interface::Events;
use crate::views::base_view::{NavigateTo, View};
use crate::views::base_view::NavigateTo::NoneView;

pub struct LoginRegisterView {
    input_mode: bool,
    username: String,
    password: String,
    error: bool,
    error_message: &'static str,
    collecting_username: bool,
    collecting_password: bool,
    is_login: bool,
    is_create: bool,
    options: Vec<&'static str>,
    navigate_to: NavigateTo,
    selected_index: usize,
    user_id: i32
}


impl LoginRegisterView {
    pub fn new() -> LoginRegisterView {
        Self {
            input_mode: false,
            username: "".to_string(),
            password: "".to_string(),
            user_id: -1,
            error: false,
            error_message: "",
            collecting_username: false,
            collecting_password: false,
            options: vec!["Login", "Register"],
            selected_index: 0,
            navigate_to:NoneView,
            is_login: false,
            is_create: false,
        }
    }


    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    fn validate_credentials(&mut self) -> bool {
        let mut error_msg: &str = "";
        let user_id: i32;
        if self.is_login {
            user_id = Manager::validate_user(self.username.as_str(), self.password.as_str());
            error_msg = "Unable to validate user, maybe wrong password?";
        }
        else if self.is_create {
            user_id =  Manager::create_user(self.username.as_str(), self.password.as_str());
            error_msg = "Unable to create user, maybe username already taken";
        }
        else {
            user_id = -1
        }


        if user_id < 0 {
            self.error = true;
            self.error_message = error_msg;
            false
        }
        else {
            Manager::login_user(user_id);
            self.user_id = user_id;
            self.navigate_to = NavigateTo::MenuView;
            true
        }
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

    fn handle_selection(&mut self) {
        /**
        *
        */
        let selection = self.get_selection();
        if selection == "Login" {
            self.is_login = true;
            self.collecting_username = true;
        } else if  selection == "Register" {
            self.is_create = true;
            self.collecting_username = true;
        }
        self.input_mode = true;
    }

    fn reset_view_state(&mut self) {
        self.error = false;
        self.input_mode = false;
        self.username = "".to_string();
        self.password = "".to_string();
        self.is_login = false;
        self.is_create = false;
        self.user_id = -1;
    }

}

impl View for LoginRegisterView {


    fn as_any(&self) -> &(dyn Any) {
        self
    }
    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        // title of view
        let mut output = String::from("\x1b[2J\x1b[H");


        // error message to display if error is true
        if self.error {
            output.push_str("\x1b[1;31mLogin ERROR\x1b[0m\r\n\r\n");
            output.push_str(self.error_message);
        } else {
            output.push_str("\x1b[1;32mLogin\x1b[0m\r\n\r\n");
        }

        // input prompts if in input mode
        if self.collecting_username && !self.error {
            let mut username_prompt = "\x1b[1;32m> Username: ".to_string();
            username_prompt.push_str(self.username.as_str());
            output.push_str(username_prompt.as_str());
        }
        else if self.collecting_password && !self.error {
            output.push_str("\x1b[1;32m> Password: ");
        }


        // if not in input mode and no error exists display login options
        else if !self.error {
            for (idx, option) in self.options.iter().enumerate() {
                if idx == self.selected_index {
                    output.push_str(&format!("\x1b[1;33m> {} \x1b[0m\r\n", option));
                } else {
                    output.push_str(&format!("  {}\r\n", option));
                }
            }
            output.push_str("\nUse ↑ (Arrow Up) / ↓ (Arrow Down)");
        }
        output
    }


    fn handle_event(&mut self, event: Events, buffer_string: String) -> Events {
        let mut view_event: Events = Events::Unknown;

        // reset error after Enter is pressed during an error
        if self.error && event == Events::Enter {
            self.reset_view_state();
        }

        // validate credentials if enter is pressed while collecting password
        else if self.input_mode && event == Events::Enter && self.collecting_password {
            match self.validate_credentials() {
                true => { view_event = Events::Authenticate; }
                false => { self.reset_view_state(); }
            }
        }

        // override username with current buffer string if collecting username
        else if self.input_mode && self.collecting_username {
            self.username = buffer_string;
            self.collecting_username = false;
            self.collecting_password = true;
        }

        // override password with current buffer string if collecting password
        else if self.input_mode && self.collecting_password {
            let password = buffer_string;
            self.password = password.clone();
            self.collecting_password = false;
            self.validate_credentials();
        }

        // handle arrow key selection when not in input mode
        else if event == Events::UpArrow && !self.input_mode {
            self.move_up();
        }

        else if event == Events::DownArrow && !self.input_mode {
            self.move_down();
        }


        // handle option selection when Enter Key when not in input mode
        else if event == Events::Enter && !self.input_mode {
            self.handle_selection();
        }

        // if the view defined a new event returns that, else returns the original event passed
        if view_event != Events::Unknown {
            view_event
        }
        else {
            event
        }

    }
}
//Events::Authenticate