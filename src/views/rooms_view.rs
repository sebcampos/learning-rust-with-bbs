use std::net::TcpStream;
use crate::views::base_view::{NavigateTo, View};
use crate::input_interface::Events;
use std::collections::HashMap;
use std::io::{Read, Write};

//#[derive(Clone)]
pub struct RoomsView {
    rooms: HashMap<String, u32>,
    display_rooms: Vec<(String, u32)>,
    selected_index: usize,
    navigate_to: NavigateTo,
    selecting_room: bool
}

impl RoomsView {
    pub fn new(rooms: HashMap<String, u32>) -> Self {
        let room_names_and_count = rooms.iter().map(|(key, &value)| (key.clone(), value)) // Clone the key and copy the value
            .collect();
        Self {
            display_rooms: room_names_and_count,
            selected_index: 0,
            rooms,
            navigate_to: NavigateTo::NoneView,
            selecting_room: true
        }
    }
    fn get_room(&self, room_name: &str) -> Option<&u32> {
        self.rooms.get(room_name)
    }

    fn update_room(&mut self, room_name: &str, online_count: u32) {
        self.rooms.insert(room_name.to_string(), online_count);
    }

    fn build_display_rooms(&mut self) {
        // Convert HashMap into a Vec of tuples (key, value)
        let mut sorted_vec: Vec<(String, u32)> = self.rooms.iter()
            .map(|(k, v)| (k.clone(), *v)) // Clone the key (String) and dereference the value (u32)
            .collect();

        // Sort by value (descending)
        sorted_vec.sort_by(|a, b| b.1.cmp(&a.1));

        self.display_rooms = sorted_vec;
    }

}

impl View for RoomsView {

    fn get_navigate_to(&self) -> &NavigateTo {
        &self.navigate_to
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H");
        output.push_str("\x1b[1;32mRooms\x1b[0m\r\n\r\n");


        // Append sorted rooms to output
        for (index, (room, count)) in self.display_rooms.iter().enumerate() {
            if index == self.selected_index && self.selecting_room {
                output.push_str(&format!("\x1b[1;33m> {}: {} online\n", room, count));
            }
            else {
                output.push_str(&format!("{}: {} online\n", room, count));
            }
        }
        //output.push_str(&format!("Search Room: "));
        output.push_str("\nUse ↑ (Arrow Up) / ↓ (Arrow Down) and Enter to select a room, use TAB to search for a room.\r\n");
        if !self.selecting_room {
            output.push_str("\x1b[1;33m> Search: ");
        }
        output
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self) {
        let display_size: usize = self.display_rooms.len();
        if display_size > 0 && self.selected_index < display_size - 1 {
            self.selected_index += 1;
        }
    }

    fn get_selection(&mut self) -> &str {
        &self.display_rooms[self.selected_index].0
    }

    fn handle_selection(&mut self, stream: &mut TcpStream) -> Events {
        Events::Exit
    }

    fn handle_event(&mut self, event: Events, stream: &mut TcpStream, buffer: Option<&[u8]>) -> Events {
        let result_event: Events;

        // if tab is hit during room selection
        // set selecting room to false and enable input mode
        if event == Events::TAB && self.selecting_room {
            self.selecting_room = false;
            result_event = Events::InputModeEnable;
        }
        //
        else if !self.selecting_room && std::str::from_utf8(buffer.unwrap()).unwrap() == "q" {
            self.selecting_room = true;
            result_event = Events::InputModeDisable;
        }
        else if !self.selecting_room && std::str::from_utf8(buffer.unwrap()).unwrap() != "q" {
            result_event = event;
        }
        else {
            result_event = event;
        }
        result_event
    }

}