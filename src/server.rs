use std::io::*;
use std::net::*;

use crate::object::Person;

pub fn main() {
    println!("Running as server");

    // Open the listener and get a connection from it
    let listener = TcpListener::bind("127.0.0.1:10104").unwrap();
    let (mut conn, _) = listener.accept().unwrap();

    // Set up the reader
    let conn_reader = BufReader::new( &mut conn);

    // Get the JSON directly from the reader
    let object: Person = serde_json::from_reader(conn_reader).unwrap();
    println!("Age: {}", object.age);
}