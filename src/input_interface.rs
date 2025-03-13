use crate::views;
use crate::views::base_view;

pub const ENTER: i32 = 13;
pub const UP_ARROW: i32 = 65;
pub const DOWN_ARROW: i32 = 66;

pub const EXIT: i32 = -1;
pub const CONTINUE: i32 = -2;



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

    pub fn get_user_action(&self, buffer: &[u8; 3]) -> i32 {
        let action: i32;
        if buffer[0] == 27 && buffer[1] == 91 {
            match buffer[2] {
                65 => action = UP_ARROW,
                66 => action = DOWN_ARROW,
                _ => action = -1
            }
        } else if buffer[0] == 13 {
            action = ENTER;
        } else {
            action = -1;
        }
        action
    }
}