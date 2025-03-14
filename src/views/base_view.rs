use std::net::TcpStream;
use crate::input_interface::Events;


#[derive(PartialEq, Eq)]
pub enum NavigateTo {
    MenuView,
    RoomsView,
    NoneView
}


pub trait View {

    fn get_navigate_to(&self) -> &NavigateTo;

    fn render(&self) -> String;
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn get_selection(&mut self) -> &str;

    fn handle_event(&mut self,  event: Events, stream: &mut TcpStream) -> Events {
        let result_event: Events;

        if event == Events::UpArrow {
            result_event = event;
            self.move_up();
        }
        else if event == Events::DownArrow {
            result_event = event;
            self.move_down();
        }
        else if event == Events::Enter {
            result_event = self.handle_selection(stream);
        }
        else {
            result_event = event;
        }
        result_event
    }
}