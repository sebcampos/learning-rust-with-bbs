use std::net::TcpStream;
use crate::input_interface;
use crate::input_interface::Events;

// Define a trait that will serve as the base class for shared behavior
pub trait View {
    fn render(&self) -> String;
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn get_selection(&mut self) -> &str;

    fn handle_event(&mut self,  event: Events, stream: &mut TcpStream);

    fn handle_selection(&mut self, stream: &mut TcpStream);
}