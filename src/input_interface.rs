use crate::{db, views};
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
    NavigateView,
    NavigatePreviousView,
    Unknown
}

impl Events {
    pub(crate) fn from_int(value: i32) -> Events {
        match value {
            65 => Events::UpArrow,
            66 => Events::DownArrow,
            13 => Events::Enter,
            _ => Events::Unknown
        }
    }
}




pub struct UserInterface {
    current_view: Box<dyn base_view::View>,
    previous_view: Option<NavigateTo>
}


impl UserInterface {

    pub fn new() -> Self {
        Self {
            current_view:  Box::new(views::menu_view::BBSMenu::new()),
            previous_view: None
        }
    }

    pub fn get_current_view(&mut self) -> &mut dyn base_view::View {
        self.current_view.as_mut()
    }

    pub fn get_user_event(&self, buffer: &[u8; 3]) -> Events {
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
            event =  Events::from_int(-1)
        }
        event
    }

    pub fn navigate_view(&mut self, manager: &db::manage::Manager) {
        let navigate_to = self.current_view.get_navigate_to();
        if *navigate_to == NavigateTo::RoomsView {
            let rooms = manager.get_rooms();
            self.current_view =  Box::new(views::rooms_view::RoomsView::new(rooms));
        }
    }

}