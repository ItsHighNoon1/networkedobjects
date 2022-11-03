use std::io::*;
use std::net::*;

use crate::object::Person;

pub fn main() {
    println!("Running as client");

    // Stream needs to be used by the buffered reader and by us so we need two references
    let stream = TcpStream::connect("localhost:10104").unwrap();

    // In stream will be owned by the buffered reader
    let in_stream = stream.try_clone().unwrap();
    let in_reader = BufReader::new(  in_stream);

    // Out stream will be owned by this method
    let mut out_stream = stream;

    // Create random object and serialize it
    let person = Person{name: "John".to_owned(), age: 16, phones: ["Phone1".to_owned(), "Phone2".to_owned()].to_vec()};
    serde_json::to_writer(&mut out_stream, &person).unwrap();

    // Send object to server
    out_stream.flush().unwrap();
}