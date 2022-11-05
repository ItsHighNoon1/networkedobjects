use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum MessageType {
    Connect,
    Disconnect,
    ClientAssign(u8),
    Text(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub origin: u8,
    pub message: MessageType,
}

pub struct Connection {
    pub id: u8,
    in_reader: BufReader<TcpStream>,
    out_writer: TcpStream,
}

impl Connection {
    pub fn new_client(stream: TcpStream) -> Result<Self, String> {
        // Set up the connection object
        match Self::new(stream, 0) {
            // On error, return error
            Err(err) => return Err(err.to_string()),

            Ok(mut new_connection) => {
                // Receive the client ID from the server
                match new_connection.get_message() {
                    Err(error) => return Err(error),
                    
                    Ok(client_assignment) => {
                        match client_assignment {
                            Message { origin: 0, message: MessageType::ClientAssign(client_id) } => {
                                // If the message was a client assign type, set the ID
                                new_connection.id = client_id;
                            },
                            _ => return Err(String::from("Received invalid CLIENT_ASSIGN message")),
                        }
                    }
                }

                // Verify that the client id is valid
                if new_connection.id == 0 {
                    return Err(String::from("Received invalid CLIENT_ASSIGN id"));
                }

                return Ok(new_connection);
            }
        }
    }

    pub fn new_server(acceptor: &TcpListener) -> Result<Self, String> {
        // Accept a new connection
        let (stream, _) = acceptor.accept().unwrap();

        // Decide on an ID for the client
        let id = 1;

        // Wrap the connection in an object
        match Self::new(stream, id) {
            // On error, return error
            Err(err) => return Err(err.to_string()),

            Ok(mut new_connection) => {
                // Send the client ID message
                let accept_message = Message {origin: 0, message: MessageType::ClientAssign(id)};
                match new_connection.send_message(&accept_message) {
                    Err(err) => return Err(err),

                    Ok(_) => return Ok(new_connection)
                }
            }
        }
    }

    fn new(stream: TcpStream, id: u8) -> Result<Self, String> {
        // Attempt to clone the stream
        match stream.try_clone() {
            // On error, return error
            Err(error) => return Err(error.to_string()),

            Ok(stream_clone) => {
                // Wrap the new stream in a reader
                let reader = BufReader::new(stream_clone);

                // Return the object
                return Ok(Self {id: id, in_reader: reader, out_writer: stream});
            }
        }
    }

    pub fn send_message(&mut self, msg: &Message) -> Result<(), String> {
        // Serialize message to string
        match serde_json::to_string(msg) {
            Err(error) => return Err(error.to_string()),

            Ok(mut json) => {
                // Add a newline as a delimiter
                json.push('\n');

                // Write to the stream
                match self.out_writer.write_all(json.as_bytes()) {
                    Err(error) => return Err(error.to_string()),
                    Ok(_) => ()
                }
            }
        }

        return Ok(());
    }

    pub fn get_message(&mut self) -> Result<Message, String> {
        // TODO danger of OOM if newline is not encountered for a long time
        let mut buf = String::new();
        match self.in_reader.read_line(&mut buf) {
            Err(error) => return Err(error.to_string()),

            Ok(len) => {
                // Truncate to len in case buffer has garbage in it
                buf.truncate(len);
                match serde_json::from_str(buf.as_str()) {
                    Ok(msg) => return Ok(msg),
                    Err(error) => return Err(error.to_string())
                }
            }
        }
    }
}