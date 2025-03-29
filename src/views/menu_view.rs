use std::any::Any;
use crate::views::base_view::{NavigateTo, View};
use crate::input_interface::Events;


pub struct BBSMenu {
    options: Vec<&'static str>,
    selected_index: usize,
    navigate_to: NavigateTo
}

impl BBSMenu {
    pub fn new() -> Self {
        Self {
            options: vec!["🚪 Rooms", "👥 People", "👨‍💻 Me", "❌ Quit"],
            selected_index: 0,
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

}


impl View for BBSMenu {
    fn as_any(&self) -> &(dyn Any) {
        self
    }

    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H"); // Clear screen + move cursor to top
        output.push_str("\x1b[1;32mWelcome to Friendly Automations Rust BBS!\x1b[0m\r\n\r\n");

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

    fn refresh_data(&mut self) {}

    fn handle_event(&mut self, event: Events, _buffer_string: String) -> Events {
        let result_event: Events;

        if event == Events::UpArrow {
            self.move_up();
        }
        else if event == Events::DownArrow {
            self.move_down()
        }
        if event != Events::Enter {
            return event;
        }
        let selection = self.get_selection();
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
            result_event = Events::Unknown;
        }
        result_event
    }
}