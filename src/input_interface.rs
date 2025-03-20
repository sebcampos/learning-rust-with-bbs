use crate::{views};
use crate::views::login_register_view::LoginRegisterView;
use crate::views::base_view::View;
use std::str;
use crate::views::base_view;
use crate::views::base_view::NavigateTo;

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
    Authenticate,
    NavigateView,
    InputModeEnable,
    InputModeDisable,
    Unknown,
    RoomJoin,
    RoomLeave,
    MessageSent
}

impl Events {
    pub(crate) fn from_int(value: i32) -> Events {
        match value {
            3  => Events::Exit, // Cntrl+C
            13 => Events::Enter,
            14 => Events::CntrlN,
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
    current_view: Box<dyn View>,
    input_mode: bool,
    user_id: i32
}


impl UserInterface {

    pub fn new() -> Self {
        Self {
            user_id: -1,
            current_view:  Box::new(LoginRegisterView::new()),
            input_mode: false
        }
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn set_user_id(&mut self) {
        let login_view = self.get_current_view();
        self.user_id = login_view.get_user_id();
    }

    pub fn set_input_mode(&mut self, active: bool) {
        self.input_mode = active;
    }

    pub fn is_in_input_mode(&self) -> bool {
        self.input_mode
    }

    pub fn get_current_view(&mut self) -> &mut dyn base_view::View {
        self.current_view.as_mut()
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

    pub fn navigate_view(&mut self) {
        let navigate_to = self.current_view.get_navigate_to();
        if *navigate_to == NavigateTo::RoomsView {
            self.current_view =  Box::new(views::rooms_view::RoomsView::new(self.user_id));
        }
        else if *navigate_to == NavigateTo::MenuView {
            self.current_view =  Box::new(views::menu_view::BBSMenu::new(self.user_id));
        }
    }

}