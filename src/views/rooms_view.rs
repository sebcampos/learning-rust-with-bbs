use std::any::Any;
use std::net::TcpStream;
use crate::views::base_view::{NavigateTo, View};
use crate::input_interface::Events;
use crate::db::manage::Manager;
use std::collections::HashMap;
use std::str;


pub struct RoomsView {
    rooms: HashMap<String, u32>,
    display_rooms: Vec<(String, u32)>,
    user_id: i32,
    selected_index: usize,
    navigate_to: NavigateTo,
    selecting_room: bool,
    searching_room: bool,
    creating_room: bool
}

impl RoomsView {
    pub fn new(user_id: i32) -> Self {
        let rooms = Manager::get_rooms();
        let room_names_and_count = rooms.iter().map(|(key, &value)| (key.clone(), value)) // Clone the key and copy the value
            .collect();
        Self {
            rooms,
            display_rooms: room_names_and_count,
            selected_index: 0,
            user_id,
            navigate_to: NavigateTo::NoneView,
            selecting_room: true,
            searching_room: false,
            creating_room: false
        }
    }
    fn get_room(&self, room_name: &str) -> Option<&u32> {
        self.rooms.get(room_name)
    }

    fn set_context_state(&mut self, state: &str) {
        if state == "selecting_room" {
            self.selecting_room = true;
            self.searching_room = false;
            self.creating_room = false;
        }
        else if state == "searching_room" {
            self.searching_room = true;
            self.selecting_room = false;
            self.creating_room = false;
        }
        else if state == "creating_room" {
            self.creating_room = true;
            self.searching_room = false;
            self.selecting_room = false;
        }

    }

    fn update_room(&mut self, room_name: &str, online_count: u32) {
        self.rooms.insert(room_name.to_string(), online_count);
    }

    fn refresh_rooms(&mut self, rooms: HashMap<String, u32>) {
        self.rooms = rooms;
        let mut sorted_vec: Vec<(String, u32)> = self.rooms.iter()
            .map(|(k, v)| (k.clone(), *v)) // Clone the key (String) and dereference the value (u32)
            .collect();

        // Sort by value (descending)
        sorted_vec.sort_by(|a, b| b.1.cmp(&a.1));

        self.display_rooms = sorted_vec;
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

    pub fn get_selection(&self) -> &str {
        &self.display_rooms[self.selected_index].0
    }

    fn get_user_id(&self) -> i32 {
        self.user_id
    }

    fn handle_selection(&mut self, stream: &mut TcpStream) -> Events {
        self.navigate_to = NavigateTo::RoomView;
        Events::RoomJoin
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
            output.push_str("\x1b[1;33m> Search (\"q\" to quit): ");
        }
        else if self.creating_room {
            output.push_str("\x1b[1;32m> Create Room (\"q\" to quit): ");
        }
        else if self.selecting_room {
            // Append sorted rooms to output
            for (index, (room, count)) in self.display_rooms.iter().enumerate() {
                if index == self.selected_index && self.selecting_room {
                    output.push_str(&format!("\x1b[1;33m> {}: {} online\x1b[0m\r\n", room, count));
                }
                else {
                    output.push_str(&format!("  {}: {} online\r\n", room, count));
                }
            }
            output.push_str("\nUse ↑ (Arrow Up) / ↓ (Arrow Down) and Enter to select a room\r\n[S] Search for a room.\r\n[C] Create Room.\r\n[N] Next Page\r\n[H] Home\r\n");
        }
        output
    }



    fn handle_event(&mut self, event: Events, buffer_string: String) -> Events {
        let result_event: Events;

        if event == Events::UpArrow {
            result_event = event;
            self.move_up();
        }
        else if event == Events::DownArrow {
            result_event = event;
            self.move_down();
        }
        // else if event == Events::Enter {
        //     result_event = self.handle_selection(stream);
        // }
        else if event == Events::KeyH && !(self.creating_room  || self.searching_room)
        {
            self.navigate_to = NavigateTo::RoomView;
            result_event = Events::NavigateView;
        }
        else if event == Events::KeyC && !self.creating_room
        {
            self.set_context_state("creating_room");
            result_event = Events::InputModeEnable;
        }
        else if event == Events::KeyS && !self.searching_room
        {
            self.set_context_state("searching_room");
            result_event = Events::InputModeEnable;
        //}

        // else if self.creating_room  || self.searching_room {
        //     let buffer_str = buffer_string.unwrap();
        //     if buffer_str.trim() == "q" {
        //         self.set_context_state("selecting_room");
        //         result_event = Events::InputModeDisable;
        //     } else if self.creating_room && buffer_str.trim() != "" {
        //         Manager::create_room(buffer_str, self.user_id.to_string());
        //         let room_query = Manager::get_rooms();
        //         self.refresh_rooms(room_query);
        //         self.set_context_state("selecting_room");
        //         result_event =  Events::InputModeDisable;;
        //     } else if self.searching_room && buffer_str.trim() != "" {
        //         let room_query = Manager::search_rooms(buffer_str);
        //         self.refresh_rooms(room_query);
        //         self.set_context_state("selecting_room");
        //         result_event =  Events::InputModeDisable;;
        //     } else {
        //         result_event = event;
        //     }
        } else {
            result_event = event;
        }
        result_event
    }

}