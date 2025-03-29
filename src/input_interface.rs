use crate::views::login_register_view::LoginRegisterView;
use crate::views::rooms_view::RoomsView;
use crate::views::room_view::RoomView;
use crate::views::base_view::View;
use std::str;
use std::sync::{Arc, Mutex};
use crate::db::manage::Manager;
use crate::views::base_view::NavigateTo;
use crate::views::menu_view::BBSMenu;
use crate::views::users_view::UsersView;
use crate::views::user_view::UserView;
use crate::views::direct_message_view::DirectMessageView;

#[derive(PartialEq, Eq)]
pub enum Events {
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Enter,
    Exit,
    TAB,
    KeyN,
    KeyS,
    KeyH,
    KeyC,
    KeyR,
    CntrlN,
    CntrlQ,
    Authenticate,
    NavigateView,
    InputModeEnable,
    SecretInputModeEnable,
    InputModeDisable,
    Unknown,
    RoomJoin,
    RoomLeave,
    DirectMessageSent,
    RoomMessageSent,
    UserView,
    BackSpace,
    SpaceBar
}

impl Events {
    pub(crate) fn from_int(value: i32) -> Events {
        match value {
            3  => Events::Exit, // Cntrl+C
            13 => Events::Enter,
            14 => Events::CntrlN,
            17 => Events::CntrlQ,
            65 => Events::UpArrow,
            66 => Events::DownArrow,
            99 => Events::KeyC,
            104 => Events::KeyH,
            110 => Events::KeyN,
            114 => Events::KeyR,
            115 => Events::KeyS,
            _ => Events::Unknown
        }
    }
}




pub struct UserInterface {
    current_view:  Arc<Mutex<dyn View>>,
    current_room: i32,
    input_mode: bool,
    user_id: i32,
    user_input: String
}


impl UserInterface {

    pub fn new() -> Self {
        let login_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(LoginRegisterView::new()));

        Self {
            user_id: -1,
            current_view:  login_view,
            input_mode: false,
            current_room: -1,
            user_input: String::new(),
        }
    }


    pub fn clear_user_input(&mut self) {
        self.user_input = String::new();
    }

    pub fn get_user_input(&self) -> String {
        self.user_input.clone()
    }

    pub fn handle_input_event(&mut self, buffer_str: &String, event: &Events) {
        if *event == Events::Exit || *event == Events::Enter {
            return;
        }

        else if *event == Events::BackSpace && self.user_input.len() > 0 {
            self.user_input.remove(self.user_input.len() - 1);
        }


        else if *event == Events::SpaceBar {
            self.user_input.push_str(" ")
        }

        else {
            let cleaned_string: String = buffer_str.chars()
                .filter(|&c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation())  // Retain only printable Unicode characters
                .collect();

            self.user_input.push_str(cleaned_string.as_str());
        }

    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn set_current_room_id(&mut self, room_id: i32) {
        self.current_room = room_id;
    }

    pub fn get_current_room_id(&self) -> i32 {
        self.current_room
    }


    pub fn join_room(&mut self) -> i32{
        let binding = self.get_current_view();
        let binding = binding.lock().unwrap();
        let rooms_view = binding
            .as_any()
            .downcast_ref::<RoomsView>().unwrap();
        let room_name = rooms_view.get_selection();
        let room_id = Manager::get_room_id_by_name(room_name.to_string());
        Manager::add_to_room_online(room_id);
        self.current_room = room_id;
        room_id
    }

    pub fn set_user_id(&mut self) {
        let binding = self.get_current_view();
        let binding = binding.lock().unwrap();
        let login_view = binding
            .as_any()
            .downcast_ref::<LoginRegisterView>().unwrap();
        let user_id = login_view.get_user_id();
        self.user_id = user_id;
    }

    pub fn set_input_mode(&mut self, active: bool) {
        self.input_mode = active;
        if !self.input_mode {
            self.user_input = String::new();
        }
    }

    pub fn is_in_input_mode(&self) -> bool {
        self.input_mode
    }

    pub fn get_current_view(&mut self) -> Arc<Mutex<dyn View>> {
        Arc::clone(&self.current_view)
    }

    pub fn get_user_event(buffer: &[u8]) -> Events {
        let event: Events;
        if buffer[0] == 27 && buffer[1] == 91 {
            event = Events::from_int(buffer[2] as i32)
        } else if buffer[0] == 13 {
            event = Events::from_int(buffer[0] as i32)
        } else if buffer[0] == b'\t' {
            event = Events::TAB
        } else if buffer[0] == 0x1b {
            event = Events::NavigateView
        } else if buffer[0] == 127 {
            event = Events::BackSpace
        } else if buffer[0] == 32 {
            event = Events::SpaceBar;
        } else {
            event =  Events::from_int(buffer[0] as i32)
        }
        event
    }

    pub fn clean_buffer(buffer: &[u8]) -> String {
        let buffer_string: &str;
        let cleaned_buffer: Vec<u8> = buffer
            .iter()                     // Iterate over the slice
            .filter(|&&x| x != 0)  // Filter out all zeros
            .copied()                   // Dereference the references to get u8 values
            .collect();
        match str::from_utf8(cleaned_buffer.as_slice()) {
            Ok(v) => {
                buffer_string = v.trim();
            },
            Err(_) => {
                buffer_string = "";
            },
        };
        buffer_string.trim().to_string()
    }

    pub fn navigate_view(&mut self) {
        let binding = self.get_current_view();
        let view = binding.lock().unwrap();
        let navigate_to = view.get_navigate_to();
        let user_id = self.get_user_id();


        if *navigate_to == NavigateTo::RoomsView {
            let rooms_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(RoomsView::new(user_id)));
            self.current_view= rooms_view

        }
        else if *navigate_to == NavigateTo::MenuView {
            let menu_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(BBSMenu::new(user_id)));
            self.current_view = menu_view

        }
        else if *navigate_to == NavigateTo::PeopleView {
            let menu_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(UsersView::new(user_id)));
            self.current_view = menu_view
        }

        else if *navigate_to == NavigateTo::MeView {
            let myself_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(UserView::new(user_id, true)));
            self.current_view =  myself_view;
        }

        else if *navigate_to == NavigateTo::UserView {
            let user_view = view.as_any().downcast_ref::<UsersView>().unwrap();
            let view_user_id = Manager::get_user_id_by_name(user_view.get_selection());
            let user_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(UserView::new(view_user_id, false)));
            self.current_view = user_view;
        }

        else if *navigate_to == NavigateTo::RoomView {
            let room_id = self.get_current_room_id();
            let room_name = Manager::get_room_name_by_id(room_id);
            let room_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(RoomView::new(room_id, room_name, user_id)));
            self.input_mode = true;
            self.current_view = room_view;
        }

        else if *navigate_to == NavigateTo::DirectMessageView {
            let user_view = view.as_any().downcast_ref::<UserView>().unwrap();
            let to_user_id = user_view.get_user_id();
            let dm_view = Arc::new(Mutex::new(DirectMessageView::new(user_id, to_user_id)));
            self.input_mode = true;
            self.current_view = dm_view;
        }
    }

}