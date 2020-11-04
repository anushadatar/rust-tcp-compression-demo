/// Test client for working with server data stream. As end-to-end testing mainly
/// happens in utils.py, this function is just for manually validating that the
/// server behaves as expected.

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

/// Based on https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
fn main() {
    match TcpStream::connect("localhost:4000") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 4000");

            let send_data = [
                83u8, 84, 82, 89, // magic number
                0, 0, // payload size
                0, 1, // ping
            ];
    
            stream.write(&send_data).unwrap();

            let mut data = [0 as u8; 8]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Unexpected reply: {}", text);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}