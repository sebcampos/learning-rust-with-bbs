mod db;
mod views;
mod input_interface;
use db::manage::Manager;
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use crate::input_interface::UserInterface;
use crate::input_interface::Events;
use crossbeam_channel::{unbounded, Sender, Receiver};

fn handle_client(mut stream: TcpStream, rx: Receiver<String>, tx_list: Arc<Mutex<Vec<Sender<String>>>>) {

    let mut user_interface = UserInterface::new();

    let disable_line_mode = [
        255, 251, 1,  // IAC WILL ECHO (Disable local echo)
        255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
        // 255, 252, 34, // IAC WONT LINEMODE (Disable Telnet linemode)
    ];
    stream.write_all(&disable_line_mode).unwrap();
    stream.flush().unwrap();
    let s_ref: &mut TcpStream = &mut stream;
    let mut buffer: Vec<u8> = vec![0; 30];

    //let mut stream_clone = s_ref.try_clone().expect("Failed to clone stream");

    // Thread to listen for broadcast messages
    let rx_thread = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            let formatted_msg = format!("{}\r\n", msg);
            println!("{}", formatted_msg);
            // if stream_clone.write_all(formatted_msg.as_bytes()).is_ok() {
            //     stream_clone.flush().unwrap();
            // } else {
            //     break; // Stop if the client disconnects
            // }
        }
    });

    let tx_clone = {
        let (client_tx, client_rx) = unbounded(); // Create a new sender for this client
        tx_list.lock().unwrap().push(client_tx); // Store in shared list
        client_rx
    };


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

        // this line causes a mutable borrow
        //let mut view = user_interface.get_current_view();
        let view_handle_event: Events;

        if is_in_input_mode {
            let buffer_string = UserInterface::clean_buffer(&buffer);
            let mut view = user_interface.get_current_view();
            view_handle_event = view.handle_event(user_event, s_ref, Some(buffer_string));
        }
        else {
            let mut view = user_interface.get_current_view();
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
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("Client {}: {}", user_interface.get_user_id(), "is online"));
            }
        } else if view_handle_event == Events::RoomJoin {
            let user_id = user_interface.get_user_id();
            let mut view = user_interface.get_current_view();
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("Client {} Joined {}", user_id, view.get_selection()));
            }
        }


        let updated_view = user_interface.get_current_view().render();
        s_ref.write_all(updated_view.as_bytes()).unwrap();
        s_ref.flush().unwrap();
        // reset the buffer
        buffer = vec![0; 30];
    }

    if user_interface.get_user_id() > 0 {
        Manager::logout_user(user_interface.get_user_id());

        // Remove the sender from the shared list
        tx_list.lock().unwrap().retain(|t| !t.is_empty());
        let tx_list_locked = tx_list.lock().unwrap();
        for tx in tx_list_locked.iter() {
            let _ = tx.send(format!("User {} offline", user_interface.get_user_id()));
        }
    }
    rx_thread.join().unwrap(); // Ensure the receiver thread stops
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2323").expect("Could not start server");
    println!("Telnet BBS started on port 2323...");
    Manager::setup_db();
    let tx_list = Arc::new(Mutex::new(Vec::new())); // Shared list of broadcasters

    for stream in listener.incoming() {
        println!("Listener incoming");
        if let Ok(stream) = stream {
            let rx_clone = {
                let (tx, rx) = unbounded(); // Create a new channel for each client
                let tx_list_locked = tx_list.clone();
                tx_list_locked.lock().unwrap().push(tx);
                rx
            };
            let tx_list_clone = Arc::clone(&tx_list);

            thread::spawn(move || {
                handle_client(stream, rx_clone, tx_list_clone);
            });
        }
    }
}