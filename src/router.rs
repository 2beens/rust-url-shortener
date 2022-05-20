use crate::handlers::Handlers;
use std::io::Read;
use std::net::TcpStream;

pub struct Router {
    suppress_logs: bool,
    is_verbose: bool,
}

impl Router {
    pub fn new(suppress_logs: bool, is_verbose: bool) -> Router {
        Router {
            suppress_logs,
            is_verbose,
        }
    }

    pub fn with_no_logs(mut self) -> Router {
        self.suppress_logs = true;
        self
    }

    pub fn with_logs(mut self) -> Router {
        self.suppress_logs = false;
        self
    }

    fn log(&self, message: String) {
        if self.suppress_logs {
            return
        }
        println!("{}", message);
    }

    pub fn route(&self, mut stream: TcpStream) {
        let mut buf = [0u8; 4096];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                if self.is_verbose {
                    self.log(String::from("+++++++++++++++++++++++++++++++++"));
                    self.log(String::from("incoming request:"));
                    self.log(req_str.to_string());
                    self.log(String::from("---------------------------------"));
                } else {
                    self.log(req_str.to_string());
                }

                let mut iter = req_str.split_whitespace();
                let method = iter.next().unwrap();
                let path = iter.next().unwrap();

                self.log(format!("==> serving [{}]: {}", method, path));
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
