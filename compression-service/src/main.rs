use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use byteorder::{BigEndian, ReadBytesExt};

mod message;
mod definitions;
mod stats;
mod utils;
mod compress;

#[allow(unused)]
fn handle_client(mut stream: TcpStream) {
    let mut stats_tracker = stats::Tracker::new();
    let mut received_data = [0 as u8; definitions::MAXIMUM_MESSAGE_SIZE]; 
    let mut send_data = [0 as u8; definitions::MAXIMUM_MESSAGE_SIZE];
    let mut received_message = message::Message::new();
    let mut send_message = message::Message::new();
    // TODO comment here
    while match stream.read(&mut received_data) {
        Ok(size) => {
            println!("Converting the input stream to a message");
            // TODO This currently incorrectly will just add everything.
            stats_tracker.add_to_bytes_read((received_data.len()));
            utils::create_input_message(&received_data, &mut received_message, &mut stats_tracker);
            utils::create_output_message(&mut received_message, &mut send_message,  &mut stats_tracker);
            send_message.to_bytes(&mut send_data);
            // TODO Make this value accurate.
            stats_tracker.add_to_bytes_sent(send_data.len());
            stream.write(&send_data).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

