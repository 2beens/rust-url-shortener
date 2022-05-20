use std::io::Write;
use std::net::TcpStream;

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
        let response =
            b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nPong!\r\n";
        match stream.write_all(response) {
            Ok(_) => println!("response [ping] sent"),
            Err(e) => println!("failed sending response [ping]: {}", e),
        }
        match stream.flush() {
            Ok(_) => println!("response [ping] flushed"),
            Err(e) => println!("failed flushing response [ping]: {}", e),
        }
    }

    pub fn handle_unknown_path(mut stream: TcpStream) {
        let response =
            b"HTTP/1.1 404\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nNot Found :(\r\n";
        match stream.write(response) {
            Ok(_) => println!("response [unknown path] sent"),
            Err(e) => println!("failed sending response [unknown path]: {}", e),
        }
    }

    pub fn handle_method_not_allowed(mut stream: TcpStream, method: &str) {
        let message = format!("HTTP/1.1 405\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nMethod {} not allowed\r\n", method);
        let response = message.as_bytes();
        match stream.write(response) {
            Ok(_) => println!("response [unknown path] sent"),
            Err(e) => println!("failed sending response [unknown path]: {}", e),
        }
    }
}
