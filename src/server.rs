use std::net::TcpListener;

use crate::network::connection::*;

pub fn main() {
    println!("Running as server");

    // Open the listener and get a connection from it
    let listener = TcpListener::bind("127.0.0.1:10104").unwrap();
    loop {
        let mut conn = Connection::new_server(&listener).unwrap();
        match conn.get_message() {
            Err(error) => println!("{}", error),

            Ok(msg) => {
                match msg {
                    Message{origin: _, message: MessageType::Text(text_data)} => println!("{}", text_data),
                    _ => println!("Some other message")
                }
            }
        }
    }
}