use std::net::{TcpStream};
use std::io::{Write};

pub struct Handlers {}

impl Handlers {
    pub fn handle_hello_world(mut stream: TcpStream) {
        let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
        match stream.write(response) {
            Ok(_) => println!("response sent"),
            Err(e) => println!("failed sending response: {}", e),
        }
    }

    pub fn handle_ping(mut stream: TcpStream) {
        let response = b"Pong!";
        match stream.write(response) {
            Ok(_) => println!("response [ping] sent"),
            Err(e) => println!("failed sending response [ping]: {}", e),
        }
    }
}
