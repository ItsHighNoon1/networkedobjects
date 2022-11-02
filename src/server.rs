use std::io::*;
use std::net::*;

pub fn main() {
    println!("Running as server");

    let listener = TcpListener::bind("127.0.0.1:10104").unwrap();
    let (mut conn, addr) = listener.accept().unwrap();

    println!("Connection from {}", addr);

    let conn_reader = BufReader::new( &mut conn);
    let http_req: Vec<_> = conn_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();

    println!("Request: {:#?}", http_req);

    let status_line = "HTTP/1.1 200 OK";
    let contents = "<!DOCTYPE html><html lang=\"en\"><head><title>A</title><body><h1>A</h1></body></html>";
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    conn.write_all(response.as_bytes()).unwrap();
}