use std::net::TcpStream;
use crate::input_interface::Events;


#[derive(PartialEq, Eq)]
pub enum NavigateTo {
    MenuView,
    RoomsView,
    PeopleView,
    MeView,
    DirectMessageView,
    UserView,
    NoneView
}


pub trait View: Send {

    fn get_navigate_to(&self) -> &NavigateTo;

    fn render(&self) -> String;
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn get_selection(&mut self) -> &str;

    fn get_user_id(&self) -> i32;

    fn handle_selection(&mut self, stream: &mut TcpStream) -> Events;

    fn handle_event(&mut self,  event: Events, stream: &mut TcpStream, buffer_string: Option<String>) -> Events {
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