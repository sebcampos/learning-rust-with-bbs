mod db;
mod views;
mod input_interface;
use db::manage::Manager;
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::input_interface::UserInterface;
use crate::input_interface::Events;

fn handle_client(mut stream: TcpStream) {

    let mut user_interface = UserInterface::new();

    // TODO move this to outside the handle_client, or make it so
    // the create statements are not called during the "new"


    let disable_line_mode = [
        255, 251, 1,  // IAC WILL ECHO (Disable local echo)
        255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
        // 255, 252, 34, // IAC WONT LINEMODE (Disable Telnet linemode)
    ];
    stream.write_all(&disable_line_mode).unwrap();
    stream.flush().unwrap();
    let s_ref: &mut TcpStream = &mut stream;
    let mut buffer: Vec<u8> = vec![0; 30];

    //let mut buffer = [u8]; // To capture arrow keys (escape sequences)
    while let Ok(n) = s_ref.read(&mut buffer) {

        if n == 0 {
            break; // Client disconnected
        }


        let user_event = UserInterface::get_user_event(&buffer);
        if user_event == Events::Exit {
            s_ref.write_all("\x1b[1;32mGoodbye!\x1b[0m\r\n\r\n".to_string().as_bytes()).unwrap();
            break;
        }
        let is_in_input_mode = user_interface.is_in_input_mode();
        let mut view = user_interface.get_current_view();
        let view_handle_event: Events;

        if is_in_input_mode {
            let buffer_string = UserInterface::clean_buffer(&buffer);
            view_handle_event = view.handle_event(user_event, s_ref, Some(buffer_string));
        }
        else {
            view_handle_event = view.handle_event(user_event, s_ref, None);
        }
        if view_handle_event == Events::Exit {
            break;
        } else if view_handle_event == Events::NavigateView {
            user_interface.navigate_view()
        } else if view_handle_event == Events::InputModeDisable {
            let disable_line_mode = [
                255, 251, 1,  // IAC WILL ECHO (Disable local echo)
                255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
            ];
            s_ref.write_all(&disable_line_mode).unwrap();
            user_interface.set_input_mode(false)
        } else if view_handle_event == Events::InputModeEnable {
            let enable_line_mode = [
                255, 252, 1,  // IAC WILL ECHO (Enable local echo)
                255, 252, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Enable line buffering)
            ];
            s_ref.write_all(&enable_line_mode).unwrap();
            user_interface.set_input_mode(true)
        } else if view_handle_event == Events::Authenticate {
            user_interface.set_user_id();
            user_interface.navigate_view();
            let disable_line_mode = [
                255, 251, 1,  // IAC WILL ECHO (Disable local echo)
                255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
            ];
            s_ref.write_all(&disable_line_mode).unwrap();
        }

        let updated_view = user_interface.get_current_view().render();
        s_ref.write_all(updated_view.as_bytes()).unwrap();
        s_ref.flush().unwrap();
        // reset the buffer
        buffer = vec![0; 30];

    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2323").expect("Could not start server");
    println!("Telnet BBS started on port 2323...");
    Manager::setup_db();


    for stream in listener.incoming() {
        println!("Listener incoming");
        if let Ok(stream) = stream {
            thread::spawn(move || {
                handle_client(stream);
            });
        }
    }
}