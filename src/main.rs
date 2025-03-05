mod db;

use std::io::{Write, Read, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
struct BBSMenu {
    options: Vec<&'static str>,
    selected_index: usize,
}

impl BBSMenu {
    fn new() -> Self {
        Self {
            options: vec!["📖 Read Messages", "✍ Post a Message", "❌ Quit"],
            selected_index: 0,
        }
    }

    fn render(&self) -> String {
        let mut output = String::from("\x1b[2J\x1b[H"); // Clear screen + move cursor to top
        output.push_str("\x1b[1;32mWelcome to Rust BBS!\x1b[0m\r\n\r\n");

        for (idx, option) in self.options.iter().enumerate() {
            if idx == self.selected_index {
                output.push_str(&format!("\x1b[1;33m> {} \x1b[0m\r\n", option)); // Highlighted selection
            } else {
                output.push_str(&format!("  {}\n", option));
            }
        }

        output.push_str("\nUse ↑ (Arrow Up) / ↓ (Arrow Down) and Enter to select.\n");
        output
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected_index < self.options.len() - 1 {
            self.selected_index += 1;
        }
    }
}

fn handle_client(mut stream: TcpStream, menu: Arc<Mutex<BBSMenu>>) {



    let disable_line_mode = [
        255, 251, 1,  // IAC WILL ECHO (Disable local echo)
        255, 251, 3,  // IAC WILL SUPPRESS_GO_AHEAD (Disable line buffering)
        // 255, 252, 34, // IAC WONT LINEMODE (Disable Telnet linemode)
    ];
    stream.write_all(&disable_line_mode).unwrap();
    stream.flush().unwrap();



    let mut buffer = [0; 3]; // To capture arrow keys (escape sequences)
    println!("handle called");
    while let Ok(n) = stream.read(&mut buffer) {
        println!("N val: {n:?}");
        if n == 0 {
            break; // Client disconnected
        }

        let mut menu_lock = menu.lock().unwrap();
        println!("BUFFER RECEIVED {buffer:?}");

        // Handle arrow key sequences (ESC [ A for up, ESC [ B for down)
        if buffer[0] == 27 && buffer[1] == 91 {
            match buffer[2] {
                65 => menu_lock.move_up(),   // Up Arrow (ESC [ A)
                66 => menu_lock.move_down(), // Down Arrow (ESC [ B)
                _ => {} // Handle other sequences if necessary
            }
        }
        // Handle Enter key
        else if buffer[0] == 13 {
            let selection = menu_lock.options[menu_lock.selected_index];
            if selection == "❌ Quit" {
                stream.write_all(b"\nGoodbye!\n").unwrap();
                break;
            } else {
                stream.write_all(format!("\nYou selected: {}\n", selection).as_bytes())
                    .unwrap();
            }
        }
        else {  }

        // After processing the input, render the updated menu
        println!("NEW MENU CREATED");
        let updated_menu = menu_lock.render();
        stream.write_all(updated_menu.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2323").expect("Could not start server");
    println!("Telnet BBS started on port 2323...");

    let menu = Arc::new(Mutex::new(BBSMenu::new()));

    for stream in listener.incoming() {
        println!("Listenter incomming");
        if let Ok(stream) = stream {
            let menu_clone = Arc::clone(&menu);
            thread::spawn(move || {
                handle_client(stream, menu_clone);
            });
        }
    }
}