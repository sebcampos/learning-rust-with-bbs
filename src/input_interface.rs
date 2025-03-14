use crate::views;
use crate::views::base_view;

pub const ENTER: i32 = 13;
pub const UP_ARROW: i32 = 65;
pub const DOWN_ARROW: i32 = 66;

pub const EXIT: i32 = -1;
pub const CONTINUE: i32 = -2;
pub const NAVIGATE_VIEW: i32 = -3;
#[derive(PartialEq, Eq)]
pub enum Events {
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Enter,
    Exit,
    NavigateView,
    Unknown
}

impl Events {
    pub(crate) fn from_int(value: i32) -> Option<Events> {
        match value {
            65 => Some(Events::UpArrow),
            66 => Some(Events::DownArrow),
            13 => Some(Events::Enter),
            _ => Some(Events::Unknown)
        }
    }
}




pub struct UserInterface {
    current_view: Box<dyn base_view::View>
}


impl UserInterface {

    pub fn new() -> Self {
        Self {
            current_view:  Box::new(views::bbs_menu::BBSMenu::new())
        }
    }

    pub fn get_current_view(&mut self) -> &mut dyn base_view::View {
        self.current_view.as_mut()
    }

    pub fn get_user_event(&self, buffer: &[u8; 3]) -> Events {
        // TODO Enter button no longer working!
        let event: Events;
        if buffer[0] == 27 && buffer[1] == 91 {
            event =  Events::from_int(buffer[2] as i32).unwrap()
        } else if buffer[0] == 13 {
            event =  Events::from_int(buffer[0] as i32).unwrap()
        } else {
            event =  Events::from_int(-1).unwrap()
        }
        event
    }
}