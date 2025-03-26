mod db;
mod views;
mod input_interface;
mod broadcast_events;

use db::manage::Manager;
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::input_interface::UserInterface;
use crate::input_interface::Events;
use crossbeam_channel::{unbounded, Sender, Receiver};
use crate::broadcast_events::handle_broadcast_event;
use crate::views::base_view::NavigateTo;

fn remove_user_from_room(room_id: i32, tx_list: Arc<Mutex<Vec<Sender<String>>>>) {
    if room_id > 0 {
        Manager::subtract_from_room_online(room_id);
        // broadcast disconnected message
        let tx_list_locked = tx_list.lock().unwrap();
        for tx in tx_list_locked.iter() {
            let _ = tx.send(format!("User left room: {}", room_id));
        }
    }
}

fn disconnect_user(user_id: i32, room_id: i32, tx_list: Arc<Mutex<Vec<Sender<String>>>>) {
    remove_user_from_room(room_id, tx_list.clone());
    if user_id > 0 {
        Manager::logout_user(user_id);
        // broadcast disconnected message
        let tx_list_locked = tx_list.lock().unwrap();
        for tx in tx_list_locked.iter() {
            let _ = tx.send(format!("User offline {}", user_id));

        }
    }
}


fn handle_client(mut stream_clone: Arc<Mutex<TcpStream>>, rx: Receiver<String>, tx_list: Arc<Mutex<Vec<Sender<String>>>>) {
    let user_interface = Arc::new(Mutex::new(UserInterface::new()));
    let user_interface_clone = Arc::clone(&user_interface);
    let disable_line_mode = [
        255, 251, 1,  // IAC WILL ECHO (Disable local echo)
        255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
    ];

    stream_clone.lock().unwrap().write_all(&disable_line_mode).unwrap();
    stream_clone.lock().unwrap().flush().unwrap();

    let thread_1_stream = Arc::clone(&stream_clone);

    let mut buffer: Vec<u8> = vec![0; 30];

    // Thread to listen for broadcast messages

    // Thread to listen for broadcast messages
    let rx_thread = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    handle_broadcast_event(msg, &user_interface_clone, &thread_1_stream);
                }
                Err(_) => {
                    println!("receiver closed.");
                    break; // Break if there's no message after timeout or if the channel is closed
                }
            }
        }
    });

    let tx_clone = {
        let (client_tx, client_rx) = unbounded(); // Create a new sender for this client
        tx_list.lock().unwrap().push(client_tx); // Store in shared list
        client_rx
    };


    loop {
        let n = stream_clone.lock().unwrap().read(&mut buffer).unwrap();

        if n == 0 {
            break; // Client disconnected
        }

        let mut ui = user_interface.lock().unwrap();

        let user_event = UserInterface::get_user_event(&buffer);
        if user_event == Events::Exit {
            stream_clone.lock().unwrap().write_all("\x1b[1;32mGoodbye!\x1b[0m\r\n\r\n".to_string().as_bytes()).unwrap();
            break;
        }
        let is_in_input_mode = ui.is_in_input_mode();

        let view_handle_event: Events;

        if is_in_input_mode {
            let buffer_string = UserInterface::clean_buffer(&buffer);
            let binding = ui.get_current_view();
            let mut view = binding.lock().unwrap();
            view_handle_event = view.handle_event(user_event, &mut *stream_clone.lock().unwrap(), Some(buffer_string));
        } else {
            let binding = ui.get_current_view();
            let mut view = binding.lock().unwrap();
            view_handle_event = view.handle_event(user_event, &mut *stream_clone.lock().unwrap(), None);
        }
        if view_handle_event == Events::Exit {
            stream_clone.lock().unwrap().write_all("\x1b[1;32mGoodbye!\x1b[0m\r\n\r\n".to_string().as_bytes()).unwrap();
            break;
        } else if view_handle_event == Events::NavigateView {
            let navigate_arg: Option<i32>;
            {
                let binding = ui.get_current_view();
                let mut view = binding.lock().unwrap();
                if *view.get_navigate_to() == NavigateTo::UserView {
                    let parsed_id: i32 = Manager::get_user_id_by_name(view.get_selection());
                    navigate_arg = Some(parsed_id);
                } else { navigate_arg = None; }
            }
            ui.navigate_view(navigate_arg);
        } else if view_handle_event == Events::InputModeDisable {
            let disable_line_mode = [
                255, 251, 1,  // IAC WILL ECHO (Disable local echo)
                255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
            ];
            stream_clone.lock().unwrap().write_all(&disable_line_mode).unwrap();
            ui.set_input_mode(false)
        } else if view_handle_event == Events::InputModeEnable {
            let enable_line_mode = [
                255, 252, 1,  // IAC WILL ECHO (Enable local echo)
                255, 252, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Enable line buffering)
            ];
            stream_clone.lock().unwrap().write_all(&enable_line_mode).unwrap();
            ui.set_input_mode(true)
        } else if view_handle_event == Events::SecretInputModeEnable {
            let enable_secret_mode = [
                255, 251, 1,
                255, 252, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Enable line buffering)
            ];
            stream_clone.lock().unwrap().write_all(&enable_secret_mode).unwrap();
            ui.set_input_mode(true)
        } else if view_handle_event == Events::Authenticate {
            ui.set_user_id();
            ui.navigate_view(None);
            let disable_line_mode = [
                255, 251, 1,  // IAC WILL ECHO (Disable local echo)
                255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
            ];
            stream_clone.lock().unwrap().write_all(&disable_line_mode).unwrap();
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                //broadcast_events::
                let _ = tx.send(format!("{{\"event_type\": \"user_login\", \"user_id\": {}}}", ui.get_user_id()));
            }
        } else if view_handle_event == Events::RoomJoin {
            let user_id = ui.get_user_id();
            let room_id;
            let room_name: String;
            {
                let binding = ui.get_current_view();
                let mut view = binding.lock().unwrap();
                room_name = view.get_selection().clone().to_string();
            }
            room_id = Manager::get_room_id_by_name(room_name.clone());
            Manager::add_to_room_online(room_id);
            ui.set_current_room_id(room_id);
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("{{\"event_type\": \"room_join\", \"user_id\": {}}}", user_id));
            }
            ui.navigate_view(None);
        } else if view_handle_event == Events::RoomLeave {
            let user_id = ui.get_user_id();
            let room_id = ui.get_current_room_id();
            Manager::subtract_from_room_online(room_id);
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("{{\"event_type\": \"room_leave\", \"user_id\": {}}}", user_id));
            }
            ui.set_current_room_id(-1);
            ui.navigate_view(None);
        } else if view_handle_event == Events::RoomMessageSent {
            let user_id = ui.get_user_id();
            let room_id = ui.get_current_room_id();
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("{{\"event_type\": \"room_message\", \"room_id\": {}}}", room_id));
            }
        }

        let updated_view = ui.get_current_view().lock().unwrap().render();
        stream_clone.lock().unwrap().write_all(updated_view.as_bytes()).unwrap();
        stream_clone.lock().unwrap().flush().unwrap();
        // reset the buffer
        buffer = vec![0; 30];
    }


    let user_id;
    let room_id;
    {
        // this block creates and releases the lock on the ui
        let ui = user_interface.lock().unwrap();
        user_id = ui.get_user_id();
        room_id = ui.get_current_room_id();
    }

    disconnect_user(user_id, room_id, tx_list.clone());

    rx_thread.join().unwrap();

    // Remove the sender from the shared list
    tx_list.lock().unwrap().retain(|t| !t.is_empty());

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2323").expect("Could not start server");
    println!("Telnet BBS started on port 2323...");
    Manager::setup_db();
    let tx_list = Arc::new(Mutex::new(Vec::new())); // Shared list of broadcasters

    for stream in listener.incoming() {
        println!("Listener incoming");
        if let Ok(stream) = stream {
            let shared_stream = Arc::new(Mutex::new(stream));
            let thread_1_stream = Arc::clone(&shared_stream);
            let rx_clone = {
                let (tx, rx) = unbounded(); // Create a new channel for each client
                let tx_list_locked = tx_list.clone();
                tx_list_locked.lock().unwrap().push(tx);
                rx
            };
            let tx_list_clone = Arc::clone(&tx_list);

            thread::spawn(move || {
                handle_client(thread_1_stream, rx_clone, tx_list_clone);
            });
        }
    }
}