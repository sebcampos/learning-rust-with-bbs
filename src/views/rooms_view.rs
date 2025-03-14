use std::net::TcpStream;
use crate::views::base_view::{NavigateTo, View};
use crate::input_interface::Events;
use std::collections::HashMap;


//#[derive(Clone)]
pub struct RoomsView {
    rooms: HashMap<String, u32>,
    display_rooms: Vec<(String, u32)>,
    selected_index: usize,
    navigate_to: NavigateTo
}

impl RoomsView {
    pub fn new(self, rooms: HashMap<String, u32>) -> Self {
        Self {
            display_rooms: rooms.iter().collect(),
            selected_index: 0,
            rooms,
            navigate_to: NavigateTo::NoneView
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
        for (room, count) in self.display_rooms.iter() {
            output.push_str(&format!("{}: {} online\n", room, count));
        }
        output
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected_index < self.display_rooms.len() - 1 {
            self.selected_index += 1;
        }
    }

    fn get_selection(&mut self) -> &str {
        self.display_rooms[self.selected_index][0]
    }

}