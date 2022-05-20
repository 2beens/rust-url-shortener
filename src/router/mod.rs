use crate::handlers::Handlers;
use std::io::Read;
use std::net::TcpStream;

pub struct Router {
    is_verbose: bool,
}

impl Router {
    pub fn new(is_verbose: bool) -> Router {
        Router { is_verbose }
    }

    pub fn route(&self, mut stream: TcpStream) {
        let mut buf = [0u8; 4096];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                if self.is_verbose {
                    println!("---------------------------------");
                    println!("incoming request:");
                    println!("{}", req_str);
                    println!("---------------------------------");
                } else {
                    println!("{}", req_str);
                }

                let mut iter = req_str.split_whitespace();
                let method = iter.next().unwrap();
                let path = iter.next().unwrap();

                println!("==> serving [{}]: {}", method, path);
                match path {
                    "/ping" => Handlers::handle_ping(stream),
                    "/hi" => {
                        if method == "GET" {
                            Handlers::handle_hello_world(stream);
                        } else {
                            Handlers::handle_method_not_allowed(stream, method);
                        }
                    }
                    _ => Handlers::handle_unknown_path(stream),
                }
            }
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }
}
