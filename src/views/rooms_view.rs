use std::any::Any;
use crate::views::base_view::{NavigateTo, View};
use crate::input_interface::Events;
use crate::db::manage::Manager;
use std::str;
use crate::input_interface::Events::Unknown;

pub struct RoomsView {
    input_mode: bool,
    input: String,
    rooms: Vec<(String, u32)>,
    user_id: i32,
    selected_index: usize,
    navigate_to: NavigateTo,
    selecting_room: bool,
    searching_room: bool,
    creating_room: bool
}

impl RoomsView {
    pub fn new(user_id: i32) -> Self {
        let rooms = Manager::get_rooms(0);
        Self {
            input_mode: false,
            input: String::new(),
            rooms,
            selected_index: 0,
            user_id,
            navigate_to: NavigateTo::NoneView,
            selecting_room: true,
            searching_room: false,
            creating_room: false
        }
    }

    fn set_context_state(&mut self, state: &str) {
        if state == "selecting_room" {
            self.selecting_room = true;
            self.searching_room = false;
            self.creating_room = false;
            self.input_mode = false;
        }
        else if state == "searching_room" {
            self.searching_room = true;
            self.selecting_room = false;
            self.creating_room = false;
            self.input_mode = false;
            self.input = String::new();
        }
        else if state == "creating_room" {
            self.creating_room = true;
            self.searching_room = false;
            self.selecting_room = false;
            self.input_mode = false;
            self.input = String::new();
        }

    }



    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self) {
        let display_size: usize = self.rooms.len();
        if display_size > 0 && self.selected_index < display_size - 1 {
            self.selected_index += 1;
        }
    }

    pub fn get_selection(&self) -> &str {
        &self.rooms[self.selected_index].0
    }


}

impl View for RoomsView {


    fn as_any(&self) -> &(dyn Any) {
        self
    }

    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H");
        output.push_str("\x1b[1;32mRooms\x1b[0m\r\n\r\n");


        if self.searching_room {
            output.push_str("\x1b[1;33m> Search (CNTRL+Q): ");
            output.push_str(self.input.as_str());

        }
        else if self.creating_room {
            output.push_str("\x1b[1;32m> Create Room (CNTRL+Q to exit): ");
            output.push_str(self.input.as_str());
        }
        else if self.selecting_room {
            // Append sorted rooms to output
            for (index, (room, count)) in self.rooms.iter().enumerate() {
                if index == self.selected_index && self.selecting_room {
                    output.push_str(&format!("\x1b[1;33m> {}: {} online\x1b[0m\r\n", room, count));
                }
                else {
                    output.push_str(&format!("  {}: {} online\r\n", room, count));
                }
            }
            output.push_str("\nUse ↑ (Arrow Up) / ↓ (Arrow Down) and Enter to select a room\r\n[S] Search for a room.\r\n[C] Create Room.\r\n[N] Next Page\r\n[H / CNTRL+Q] Home\r\n");
        }
        output
    }

    fn refresh_data(&mut self) {
        if self.searching_room {
            self.rooms = Manager::search_rooms(self.input.to_string(), 0);
        }

        else {
            self.rooms = Manager::get_rooms(0);
        }
    }

    fn handle_event(&mut self, event: Events, buffer_string: String) -> Events {
        let mut result_event: Events = Unknown;

        if event == Events::CntrlQ && !self.input_mode {
            self.navigate_to = NavigateTo::MenuView;
            result_event = Events::NavigateView
        }

        else if event == Events::CntrlQ && self.input_mode {
            self.set_context_state("selecting_room");
            result_event = Events::InputModeDisable
        }


        else if event == Events::UpArrow && !self.input_mode{
            self.move_up();
        }

        else if event == Events::DownArrow && !self.input_mode{
            self.move_down();
        }

        else if event == Events::Enter && !self.input_mode && self.rooms.len() > 0 {
            self.navigate_to = NavigateTo::RoomView;
            result_event = Events::RoomJoin;
        }

        else if event == Events::KeyH && !(self.creating_room  || self.searching_room) {
            self.navigate_to = NavigateTo::MenuView;
            result_event = Events::NavigateView;
        }

        else if event == Events::KeyC && !self.creating_room && !self.input_mode{
            self.set_context_state("creating_room");
            self.input_mode = true;
            result_event = Events::InputModeEnable;
        }

        else if event == Events::KeyS && !self.searching_room && !self.input_mode {
            self.set_context_state("searching_room");
            self.input_mode = true;
            result_event = Events::InputModeEnable;
        }


        else if self.input_mode && event != Events::Enter {
            self.input = buffer_string;
        }

        else if self.input_mode && event == Events::Enter && self.creating_room && self.input.trim() != ""{
            Manager::create_room(self.input.to_string(), self.user_id.to_string());
            self.refresh_data();
            self.set_context_state("selecting_room");
            result_event =  Events::InputModeDisable;
        }

        else if self.input_mode && event == Events::Enter && self.searching_room && self.input.trim() != "" {
            self.refresh_data();
            self.set_context_state("selecting_room");
            result_event =  Events::InputModeDisable;
        }
        else {
            result_event = event;
        }
        result_event
    }

}