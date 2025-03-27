use std::any::Any;
use std::net::TcpStream;
use crate::views::base_view::{NavigateTo, View};
use crate::input_interface::Events;
use std::io::{Write};

//#[derive(Clone)]
pub struct BBSMenu {
    options: Vec<&'static str>,
    user_id: i32,
    selected_index: usize,
    navigate_to: NavigateTo
}

impl BBSMenu {
    pub fn new(user_id: i32) -> Self {
        Self {
            options: vec!["🚪 Rooms", "👥 People", "👨‍💻 Me", "❌ Quit"],
            selected_index: 0,
            user_id,
            navigate_to: NavigateTo::NoneView
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

    fn get_user_id(&self) -> i32 {
        self.user_id
    }


    fn handle_selection(&mut self, stream: &mut TcpStream) -> Events {
        let selection = self.get_selection();
        let result_event: Events;
        if selection == "❌ Quit" {
            result_event = Events::Exit;
        } else if  selection == "🚪 Rooms" {
            self.navigate_to = NavigateTo::RoomsView;
            result_event = Events::NavigateView;
        } else if  selection == "👥 People" {
            self.navigate_to = NavigateTo::PeopleView;
            result_event = Events::NavigateView;
        } else if  selection == "👨‍💻 Me" {
            self.navigate_to = NavigateTo::MeView;
            result_event = Events::NavigateView;
        } else {
            stream.write_all(format!("\nYou selected: {}\n", selection).as_bytes())
                .unwrap();
            result_event = Events::NavigateView;
        }
        result_event
    }

}

// Implement the `Menu` trait for `BBSMenu`
impl View for BBSMenu {
    fn as_any(&self) -> &(dyn Any) {
        self
    }

    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H"); // Clear screen + move cursor to top
        output.push_str("\x1b[1;32mWelcome to Rust BBS!\x1b[0m\r\n\r\n");

        for (idx, option) in self.options.iter().enumerate() {
            if idx == self.selected_index {
                output.push_str(&format!("\x1b[1;33m> {} \x1b[0m\r\n", option)); // Highlighted selection
            } else {
                output.push_str(&format!("  {}\r\n", option));
            }
        }

        output.push_str("\nUse ↑ (Arrow Up) / ↓ (Arrow Down) and Enter to select.\r\n");
        output
    }

    fn refresh_data(&mut self) {
        todo!()
    }

    fn handle_event(&mut self, event: Events, buffer_string: String) -> Events {
        todo!()
    }
}