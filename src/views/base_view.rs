use std::net::TcpStream;

// Define a trait that will serve as the base class for shared behavior
pub trait View {
    fn render(&self) -> String;
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn get_selection(&mut self) -> &str;

    fn handle_action(&mut self, action: i32, stream: &mut TcpStream) -> i32;
}