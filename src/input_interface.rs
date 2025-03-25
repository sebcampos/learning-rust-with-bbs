use crate::views::login_register_view::LoginRegisterView;
use crate::views::rooms_view::RoomsView;
use crate::views::room_view::RoomView;
use crate::views::base_view::View;
use std::str;
use std::sync::{Arc, Mutex};
use crate::views::base_view::NavigateTo;
use crate::views::menu_view::BBSMenu;
use crate::views::users_view::UsersView;
use crate::views::user_view::UserView;

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
    MessageSent,
    UserView
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
}


impl UserInterface {

    pub fn new() -> Self {
        let login_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(LoginRegisterView::new()));

        Self {
            user_id: -1,
            current_view:  login_view,
            input_mode: false,
            current_room: -1,
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

    pub fn set_user_id(&mut self) {
        let login_view = self.get_current_view();
        let user_id = login_view.lock().unwrap().get_user_id();
        self.user_id = user_id;
    }

    pub fn set_input_mode(&mut self, active: bool) {
        self.input_mode = active;
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

    pub fn navigate_view(&mut self, view_user_id: Option<i32>) {
        let user_id = self.get_user_id();
        let view_user_id = view_user_id.unwrap_or(-1);
        let binding = self.get_current_view();
        let view = binding.lock().unwrap();
        let navigate_to = view.get_navigate_to();


        // TODO these views being singleton might be a problem

        if *navigate_to == NavigateTo::RoomsView {
            let rooms_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(RoomsView::new(user_id)));
            //drop(view); // Explicitly unlocks the MutexGuard here
            self.current_view= rooms_view
            //self.current_view =  Box::new(views::rooms_view::RoomsView::new(self.user_id));
        }
        else if *navigate_to == NavigateTo::MenuView {
            let menu_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(BBSMenu::new(user_id)));
            //drop(view); // Explicitly unlocks the MutexGuard here
            self.current_view = menu_view
            //self.current_view =  Box::new(views::menu_view::BBSMenu::new(self.user_id));
        }
        else if *navigate_to == NavigateTo::PeopleView {
            let menu_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(UsersView::new(user_id)));
            //drop(view); // Explicitly unlocks the MutexGuard here
            self.current_view = menu_view
            //self.current_view =  Box::new(views::menu_view::BBSMenu::new(self.user_id));
        }

        else if *navigate_to == NavigateTo::MeView {
            let myself_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(UserView::new(user_id, true)));
            //drop(view); // Explicitly unlocks the MutexGuard here

            self.current_view =  myself_view;
        }

        else if *navigate_to == NavigateTo::UserView {
            let user_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(UserView::new(view_user_id, false)));
            self.current_view = user_view;
        }

        else if *navigate_to == NavigateTo::RoomView {
            let user_view: Arc<Mutex<dyn View>> = Arc::new(Mutex::new(RoomView::new(view_user_id)));
            self.current_view = user_view;
        }
    }

}