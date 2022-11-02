use std::io::*;
use std::net::*;

pub fn main() {
    println!("Running as client");

    // Steam needs to be used by the buffered reader and by us so we need two references
    let mut stream = TcpStream::connect("localhost:10104").unwrap();

    // In stream will be owned by the buffered reader
    let in_stream = stream.try_clone().unwrap();

    // Out stream will be owned by this method
    let out_stream = &mut stream;

    let conn_reader = BufReader::new(  in_stream);

    out_stream.write_all("Request bytes\n\n".as_bytes()).unwrap();
    out_stream.flush().unwrap();
    for line in conn_reader.lines() {
        println!("{}", line.unwrap());
    }
}