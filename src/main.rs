mod db;
mod views;
mod input_interface;
mod broadcast_events;

use db::manage::Manager;
use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::input_interface::UserInterface;
use crate::input_interface::Events;
use crossbeam_channel::{unbounded, Sender, Receiver};
use crate::broadcast_events::handle_broadcast_event;
use crate::views::direct_message_view::DirectMessageView;
use crate::views::user_view::UserView;

fn remove_user_from_room(room_id: i32, tx_list: Arc<Mutex<Vec<Sender<String>>>>) {
    if room_id > 0 {
        Manager::subtract_from_room_online(room_id);
        // broadcast disconnected message
        let tx_list_locked = tx_list.lock().unwrap();
        for tx in tx_list_locked.iter() {
            let _ = tx.send(format!("{{\"event_type\": \"room_leave\", \"user_id\": {}}}", room_id));
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
            let _ = tx.send(format!("{{\"event_type\": \"logout\", \"user_id\": {}}}", user_id));

        }
    }
    else {
        let tx_list_locked = tx_list.lock().unwrap();
        for tx in tx_list_locked.iter() {
            let _ = tx.send("{\"event_type\": \"anon_logout\"}".to_string());

        }
    }
}

fn enable_secret_mode(stream: &Arc<Mutex<TcpStream>>) {
    let enable_secret_mode = [
        255, 251, 1,
        255, 252, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Enable line buffering)
    ];
    stream.lock().unwrap().write_all(&enable_secret_mode).unwrap();
    stream.lock().unwrap().flush().expect("TODO: panic message");
}


fn disable_line_mode(stream: &Arc<Mutex<TcpStream>>) {
    let disable_line_mode = [
        255, 251, 1,  // IAC WILL ECHO (Disable local echo)
        255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
    ];

    stream.lock().unwrap().write_all(&disable_line_mode).unwrap();
    stream.lock().unwrap().flush().unwrap();
}

fn enable_line_mode(mut stream: &Arc<Mutex<TcpStream>>) {
    let enable_line_mode = [
        255, 252, 1,  // IAC WILL ECHO (Enable local echo)
        255, 252, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Enable line buffering)
    ];
    stream.lock().unwrap().write_all(&enable_line_mode).unwrap();
    stream.lock().unwrap().flush().expect("TODO: panic message");
}


fn output_goodbye_message(stream: &Arc<Mutex<TcpStream>>) {
    stream.lock().unwrap().write_all("\x1b[1;32mGoodbye!\x1b[0m\r\n\r\n".to_string().as_bytes()).unwrap();
}

fn handle_client(mut stream_clone: Arc<Mutex<TcpStream>>, rx: Receiver<String>, tx_list: Arc<Mutex<Vec<Sender<String>>>>) {

    // create a new thread safe instance of user interface
    let user_interface = Arc::new(Mutex::new(UserInterface::new()));

    // clone to share with broadcast thread
    let user_interface_clone = Arc::clone(&user_interface);

    disable_line_mode(&stream_clone);

    let broadcast_stream_clone = Arc::clone(&stream_clone);

    let mut buffer: Vec<u8> = vec![0; 30];
    let mut stop_receiver = Arc::new(Mutex::new(false));
    let stop_flag = Arc::clone(&stop_receiver);

    // Thread to listen for broadcast messages and update ui via the shared stream object
    let rx_thread = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    let stop = stop_flag.lock().unwrap();
                    if *stop {
                        break;
                    }
                    let res = handle_broadcast_event(msg, &user_interface_clone, &broadcast_stream_clone);
                    if res == -1 {
                        break;
                    }
                }
                Err(e) => {
                    println!("receiver closed.");
                    println!("error: {}", e);
                    break; // Break if there's no message after timeout or if the channel is closed
                }
            }
        }
    });



    loop {
        match stream_clone.lock().unwrap().read(&mut buffer) {
            Ok(0) => {
                // The client has gracefully disconnected
                println!("Client disconnected (EOF).");
                break; // Break the loop to close the connection
            }
            Ok(n) => n, // Data was read successfully
            Err(_) => {
                0 // Exit on error (client may have closed the connection)
            }
        };

        // unlock the user interface
        let mut ui = user_interface.lock().unwrap();
        let user_event = UserInterface::get_user_event(&buffer);
        if user_event == Events::Exit {
            output_goodbye_message(&stream_clone);
            break;
        }

        // collect buffer as string
        let buffer_string = UserInterface::clean_buffer(&buffer);

        if ui.is_in_input_mode() && user_event != Events::UpArrow && user_event != Events::DownArrow {
            ui.handle_input_event(&buffer_string, &user_event)
        }

        let binding = ui.get_current_view();
        //let mut view = binding.lock().unwrap();

        let view_handle_event;
        if ui.is_in_input_mode() {
            let mut view = binding.lock().unwrap();
            view_handle_event = view.handle_event(user_event, ui.get_user_input());
            if view_handle_event == Events::Enter  || view_handle_event == Events::RoomMessageSent  || view_handle_event == Events::DirectMessageSent {
                ui.clear_user_input()
            }
        }
        else {
            let mut view = binding.lock().unwrap();
            view_handle_event = view.handle_event(user_event, buffer_string);
        }

        // handle default exit event
        if view_handle_event == Events::Exit {
            output_goodbye_message(&stream_clone);
            break;
        }

        // Handle view navigation event
        else if view_handle_event == Events::NavigateView {
            ui.navigate_view();
        }


        else if view_handle_event == Events::InputModeDisable {
            ui.set_input_mode(false);
        }



        else if view_handle_event == Events::InputModeEnable {
            ui.set_input_mode(true);
        }


        else if view_handle_event == Events::Authenticate {
            ui.set_user_id();
            ui.navigate_view();
            ui.set_input_mode(false);
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("{{\"event_type\": \"user_login\", \"user_id\": {}}}", ui.get_user_id()));
            }
        }


        else if view_handle_event == Events::RoomJoin {
            let user_id = ui.get_user_id();
            let room_id: i32 = ui.join_room();
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("{{\"event_type\": \"room_join\", \"user_id\": {},  \"room_id\": {}}}", user_id, room_id));
            }
            ui.navigate_view();
        }



        else if view_handle_event == Events::RoomLeave {
            disable_line_mode(&stream_clone);
            let user_id = ui.get_user_id();
            let room_id = ui.get_current_room_id();
            Manager::subtract_from_room_online(room_id);
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("{{\"event_type\": \"room_leave\", \"user_id\": {}}}", user_id));
            }
            ui.set_current_room_id(-1);
            ui.navigate_view();
        }


        else if view_handle_event == Events::RoomMessageSent {
            enable_line_mode(&stream_clone);
            let room_id = ui.get_current_room_id();
            let tx_list_locked = tx_list.lock().unwrap();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("{{\"event_type\": \"room_message\", \"room_id\": {}}}", room_id));
            }
            disable_line_mode(&stream_clone);
        }

        else if view_handle_event == Events::DirectMessageSent {
            let tx_list_locked = tx_list.lock().unwrap();
            let mut view = binding.lock().unwrap();
            let user_view = view.as_any().downcast_ref::<DirectMessageView>().unwrap();
            let to_user_id = user_view.to_user_id();
            for tx in tx_list_locked.iter() {
                let _ = tx.send(format!("{{\"event_type\": \"direct_message\", \"user_id\": {}, \"to_user_id\": {}}}", ui.get_user_id(), to_user_id));
            }

        }

        let mut view = binding.lock().unwrap();
        let updated_view = view.render();
        stream_clone.lock().unwrap().write_all(updated_view.as_bytes()).unwrap();
        stream_clone.lock().unwrap().flush().unwrap();

        buffer = vec![0; 30];
    }


    let user_id;
    let room_id;
    {

        let mut ui = user_interface.lock().unwrap();
        user_id = ui.get_user_id();
        room_id = ui.get_current_room_id();
    }
    {
        let mut stop = stop_receiver.lock().unwrap();
        *stop = true;
    }
    disconnect_user(user_id, room_id, tx_list.clone());
    rx_thread.join().unwrap();

}

fn main() {

    // begins a listener for tcp connections
    let listener = TcpListener::bind("0.0.0.0:2323").expect("Could not start server");
    println!("Telnet BBS started on port 2323...");


    // runs "create if not exists" sql commands to set up db
    Manager::setup_db();

    // shared list of broadcasters
    let tx_list = Arc::new(Mutex::new(Vec::new()));


    // for every incoming connection
    for stream in listener.incoming() {


        if let Ok(stream) = stream {

            stream.set_read_timeout(Some(Duration::new(1, 0))).expect("TODO: panic message");


            // create a Mutex shared stream so it can be shared between 2 threads
            let shared_stream = Arc::new(Mutex::new(stream));

            // clone the shared stream
            let stream_clone = Arc::clone(&shared_stream);


            // Create a new broadcast client/receiver for each new client stream
            let rx_clone = {
                let (tx, rx) = unbounded();
                let tx_list_locked = tx_list.clone();
                tx_list_locked.lock().unwrap().push(tx);
                rx
            };

            // clone the shared broadcast list
            let tx_list_clone = Arc::clone(&tx_list);



            // pass the cloned stream, receiver, and shared broadcast list to the main handler
            thread::spawn(move || {
                handle_client(stream_clone, rx_clone, tx_list_clone);
            });
        }
    }
}