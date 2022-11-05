use std::net::*;

use crate::network::connection::*;

pub fn main() {
    println!("Running as client");

    // Connect to the server
    let stream = TcpStream::connect("localhost:10104").unwrap();

    // Wrap the stream in a connection object
    let mut conn = Connection::new_client(stream).unwrap();

    // Check what the ID is
    println!("ID: {}", conn.id);

    // Send a message
    let msg = Message {origin: conn.id, message: MessageType::Text(String::from("Some text"))};
    match conn.send_message(&msg) {
        Err(error) => println!("{}", error),

        Ok(_) => ()
    }
}