use http::StatusCode;
use redis::RedisError;

use crate::delete_handler::DeleteHandler;
use crate::get_all_handler::GetAllHandler;
use crate::handlers::Handlers;
use crate::link_handler::LinkHandler;
use crate::new_handler::NewHandler;
use log::{debug, error};
use std::io::Read;
use std::net::TcpStream;

pub struct Router {
    suppress_logs: bool,
    is_verbose: bool,

    // handlers
    link_handler: LinkHandler,
    new_handler: NewHandler,
    get_all_handler: GetAllHandler,
    delete_handler: DeleteHandler,
}

impl Router {
    pub fn new(
        redis_conn_string: String,
        suppress_logs: bool,
        is_verbose: bool,
    ) -> Result<Router, RedisError> {
        let link_handler = LinkHandler::new(&redis_conn_string)?;
        let new_handler = NewHandler::new(&redis_conn_string)?;
        let delete_handler = DeleteHandler::new(&redis_conn_string)?;
        let get_all_handler = crate::get_all_handler::GetAllHandler::new(&redis_conn_string)?;
        Ok(Router {
            suppress_logs,
            is_verbose,
            link_handler,
            new_handler,
            get_all_handler,
            delete_handler,
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
        debug!("{}", message);
    }

    pub fn route(&mut self, mut stream: TcpStream) {
        let mut buf = [0u8; 4096];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                if self.is_verbose {
                    self.log(String::from("+++++++++++++++++++++++++++++++++"));
                    self.log(String::from(format!(
                        "incoming request, len [{}]:",
                        req_str.len()
                    )));
                    self.log(format!("[[{}]]", req_str.to_string()));
                    self.log(String::from("---------------------------------"));
                } else {
                    self.log(req_str.to_string());
                }

                if req_str == "" {
                    self.log(String::from("received an empty request"));
                    Handlers::handle_unknown_path(stream);
                    return;
                }

                let mut iter = req_str.split_whitespace().take(2);
                let method = iter.next().unwrap();
                let path = iter.next().unwrap();

                self.log(format!("==> serving [{}]: {}", method, path));

                self.route_path(stream, method, path, &req_str);
            }
            Err(e) => error!("Unable to read stream: {}", e),
        }
    }

    fn route_path(&mut self, stream: TcpStream, method: &str, path: &str, req_str: &str) {
        // get link and redirect to it
        if path.starts_with("/l/") {
            if method != "GET" {
                Handlers::handle_method_not_allowed(stream, method);
                return;
            }

            self.link_handler.handle_link(stream, path);
            return;
        } else if path.starts_with("/delete") {
            if method == "OPTIONS" {
                Handlers::respond_options_ok(stream, path, "DELETE");
                return;
            } else if method != "DELETE" {
                let session_cookie = get_req_header("X-SERJ-TOKEN", req_str);
                if session_cookie == "" {
                    Handlers::handle_unauthorized(stream);
                    return;
                }

                Handlers::handle_method_not_allowed(stream, method);
                return;
            }

            self.delete_handler.handle_delete(stream, path);
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
                if method == "OPTIONS" {
                    Handlers::respond_options_ok(stream, path, "POST");
                    return;
                } else if method != "POST" {
                    Handlers::handle_method_not_allowed(stream, method);
                    return;
                }

                let session_cookie = get_req_header("X-SERJ-TOKEN", req_str);
                if session_cookie == "" {
                    Handlers::handle_unauthorized(stream);
                    return;
                }

                let post_body;
                let mut iter = req_str.lines().rev().take(1);
                if let Some(body) = iter.next() {
                    post_body = String::from(body.trim_matches(char::from(0)));
                } else {
                    Handlers::respond_with_status_code(
                        stream,
                        StatusCode::BAD_REQUEST.as_u16(),
                        String::from("missing request body"),
                    );
                    return;
                }

                self.new_handler.handle_new(stream, post_body);
            }
            "/all" => {
                if method == "OPTIONS" {
                    Handlers::respond_options_ok(stream, path, "GET");
                } else if method == "GET" {
                    let session_cookie = get_req_header("X-SERJ-TOKEN", req_str);
                    if session_cookie == "" {
                        Handlers::handle_unauthorized(stream);
                        return;
                    }

                    self.get_all_handler.handle_get_all(stream);
                } else {
                    Handlers::handle_method_not_allowed(stream, method);
                }
            }
            _ => Handlers::handle_unknown_path(stream),
        }
    }
}

fn get_req_header(header: &str, req_str: &str) -> String {
    for line in req_str.lines() {
        let mut next_line = line.trim_start();
        next_line = next_line.trim_end();

        if !next_line.starts_with(header) {
            continue;
        }

        let header_parts: Vec<&str> = next_line.split_terminator(":").collect();
        if header_parts.len() != 2 {
            continue;
        }

        return header_parts[1].trim_start().trim_end().to_string();
    }

    return "".to_string();
}

#[cfg(test)]
mod tests {
    use crate::router::get_req_header;

    #[test]
    fn test_get_req_header() {
        let example_req = r#"
            POST /new HTTP/1.1
            Host: localhost:8080
            User-Agent: curl/7.83.1
            Accept: */*
            Cookie: sessionkolacic=abcdef
            X-SERJ-TOKEN: blabla
            Content-Length: 20
            Content-Type: application/x-www-form-urlencoded

            url=http://www.st.rs
        "#;
        let got_cookie = get_req_header("X-SERJ-TOKEN", example_req);
        assert_eq!(got_cookie, "blabla");
    }
}
