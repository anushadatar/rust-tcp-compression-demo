/// Run the TCP server, process incoming messages and send responses.
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

mod compress;
mod definitions;
mod message;
mod stats;
mod utils;

/// Manage the incoming stream by processing incoming messages, sending
/// responses, and maintaining statistics associated with those operations.
fn handle(mut stream: TcpStream) {
    // Manages the statistics tracking associated with this server.
    let mut stats_tracker = stats::Tracker::new();
    // Buffer associated with data received over the TCP stream.
    let mut received_data = [0 as u8; definitions::MAXIMUM_MESSAGE_SIZE];
    // Message struct associated with received data.
    let mut received_message = message::Message::new();
    // Buffer associated with data to send over the TCP stream.
    let mut send_data = [0 as u8; definitions::MAXIMUM_MESSAGE_SIZE];
    // Message struct associated with the data to send over the TCP stream.
    let mut send_message = message::Message::new();
    while match stream.read(&mut received_data) {
        Ok(_size) => {
            utils::create_input_message(&received_data, &mut received_message, &mut stats_tracker);
            utils::create_output_message(
                &mut received_message,
                &mut send_message,
                &mut stats_tracker,
            );
            let output_size = send_message.to_bytes(&mut send_data);
            stats_tracker.add_to_bytes_sent(output_size as u32);
            stream.write(&send_data).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred with the TCP stream, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

/// Main method associated with TCP listener initialization.
/// Used this tutorial as a starting point: https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
