use crate::views;
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
    NavigateView,
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
    current_view: Box<dyn base_view::View>
}


impl UserInterface {

    pub fn new() -> Self {
        Self {
            current_view:  Box::new(views::menu_view::BBSMenu::new())
        }
    }

    pub fn get_current_view(&mut self) -> &mut dyn base_view::View {
        self.current_view.as_mut()
    }

    pub fn get_user_event(&self, buffer: &[u8; 3]) -> Events {
        let event: Events;
        if buffer[0] == 27 && buffer[1] == 91 {
            event =  Events::from_int(buffer[2] as i32)
        } else if buffer[0] == 13 {
            event =  Events::from_int(buffer[0] as i32)
        } else {
            event =  Events::from_int(-1)
        }
        event
    }

    pub fn navigate_view(&mut self, view: NavigateTo) {
        if view == NavigateTo::RoomsView {
            &self.current_view:  Box::new(views::menu_view::BBSMenu::new())
        }
    }

}