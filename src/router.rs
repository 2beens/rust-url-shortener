use redis::RedisError;

use crate::get_all_handler::GetAllHandler;
use crate::handlers::Handlers;
use crate::link_handler::LinkHandler;
use crate::new_handler::NewHandler;
use std::io::Read;
use std::net::TcpStream;

pub struct Router {
    suppress_logs: bool,
    is_verbose: bool,

    // handlers
    link_handler: LinkHandler,
    new_handler: NewHandler,
    get_all_handler: GetAllHandler,
}

impl Router {
    pub fn new(
        redis_conn_string: String,
        suppress_logs: bool,
        is_verbose: bool,
    ) -> Result<Router, RedisError> {
        let link_handler = LinkHandler::new(&redis_conn_string)?;
        let new_handler = NewHandler::new(&redis_conn_string)?;
        let get_all_handler = crate::get_all_handler::GetAllHandler::new(&redis_conn_string)?;
        Ok(Router {
            suppress_logs,
            is_verbose,
            link_handler,
            new_handler,
            get_all_handler,
        })
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
            return;
        }
        println!("{}", message);
    }

    pub fn route(&mut self, mut stream: TcpStream) {
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

                let mut iter = req_str.split_whitespace().take(2);
                let method = iter.next().unwrap();
                let path = iter.next().unwrap();

                let mut post_body = String::from("");
                let mut iter = req_str.lines().rev().take(1);
                if let Some(body) = iter.next() {
                    post_body = String::from(body);
                }

                self.log(format!("==> serving [{}]: {}", method, path));

                // get link and redirect to it
                if path.starts_with("/l/") {
                    if method != "GET" {
                        Handlers::handle_method_not_allowed(stream, method);
                        return;
                    }

                    self.link_handler.handle_link(stream, path);
                    return;
                }

                match path {
                    "/ping" => Handlers::handle_ping(stream),
                    "/hi" => {
                        if method == "GET" {
                            Handlers::handle_hello_world(stream);
                        } else {
                            Handlers::handle_method_not_allowed(stream, method);
                        }
                    }
                    "/new" => {
                        if method == "POST" {
                            self.new_handler.handle_new(stream, post_body);
                        } else {
                            Handlers::handle_method_not_allowed(stream, method);
                        }
                    }
                    "/all" => {
                        if method == "GET" {
                            self.get_all_handler.handle_get_all(stream);
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
