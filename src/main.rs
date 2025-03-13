mod db;
mod views;
mod input_interface;

use std::io::{Write, Read, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::input_interface::UserInterface;

fn handle_client(mut stream: TcpStream) {

    let mut user_interface = UserInterface::new();

    let disable_line_mode = [
        255, 251, 1,  // IAC WILL ECHO (Disable local echo)
        255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
        // 255, 252, 34, // IAC WONT LINEMODE (Disable Telnet linemode)
    ];
    stream.write_all(&disable_line_mode).unwrap();
    stream.flush().unwrap();
    let s_ref: &mut TcpStream = &mut stream;

    let mut buffer = [0; 3]; // To capture arrow keys (escape sequences)
    while let Ok(n) = s_ref.read(&mut buffer) {
        println!("N val: {n:?}");

        if n == 0 {
            break; // Client disconnected
        }

        //let mut interface_lock = user_interface.lock().unwrap();

        let action = user_interface.get_user_action(&buffer);

        let mut view = user_interface.get_current_view();
        let action_result = view.handle_action(action, s_ref);
        if action_result == input_interface::EXIT {
            break;
        }

        let updated_view = user_interface.get_current_view().render();
        s_ref.write_all(updated_view.as_bytes()).unwrap();
        s_ref.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2323").expect("Could not start server");
    println!("Telnet BBS started on port 2323...");



    for stream in listener.incoming() {
        println!("Listenter incomming");
        if let Ok(stream) = stream {
            thread::spawn(move || {
                handle_client(stream);
            });
        }
    }
}