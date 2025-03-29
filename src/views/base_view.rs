use std::any::Any;
use crate::input_interface::Events;


#[derive(PartialEq, Eq)]
pub enum NavigateTo {
    MenuView,
    RoomsView,
    RoomView,
    PeopleView,
    MeView,
    DirectMessageView,
    UserView,
    NoneView
}


pub trait View: Send {

    fn as_any(&self) -> &(dyn Any);

    fn get_navigate_to(&self) -> &NavigateTo;

    fn render(&self) -> String;

    fn refresh_data(&mut self) {}

    fn handle_event(&mut self,  event: Events, buffer_string: String) -> Events;

}