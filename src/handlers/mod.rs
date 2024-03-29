use log::{debug, error};
use std::io::Write;
use std::net::TcpStream;

pub struct Handlers {}

impl Handlers {
    pub fn handle_redirect(mut stream: TcpStream, url: String) {
        let content = r#"<html>
<head>
    <title>Moved</title>
</head>
<body>
    =Moved=
    <p>This page has moved.</p>
</body>
</html>
"#;
        let content_len = content.len();
        let response = format!(
            r#"HTTP/1.1 301 Moved Permanently
content-type: text/html; charset=UTF-8
Content-Length: {content_len}
Location: {url}
Content-Type: text/html

{content}"#
        );

        match stream.write(response.as_bytes()) {
            Ok(_) => debug!("redirect response sent: {}", response),
            Err(e) => error!("failed sending redirect response: {}", e),
        }
        match stream.flush() {
            Ok(_) => debug!("response [redirect] flushed"),
            Err(e) => error!("failed flushing response [redirect]: {}", e),
        }
    }

    pub fn respond_with_status_code(mut stream: TcpStream, code: u16, message: String) {
        let response = format!(
            "HTTP/1.1 {code}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{message}\r\n"
        );
        match stream.write_all(response.as_bytes()) {
            Ok(_) => debug!("response sent"),
            Err(e) => error!("failed sending response: {}", e),
        }
        match stream.flush() {
            Ok(_) => debug!("response flushed"),
            Err(e) => error!("failed flushing json response: {}", e),
        }
    }

    pub fn respond_options_ok(mut stream: TcpStream, path: &str, allowed_method: &str) {
        let response = String::from(
            format!("HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: {},OPTIONS\r\nAccess-Control-Allow-Headers: *\r\n\r\n<html><body>OK</body></html>\r\n", allowed_method)
        );
        match stream.write_all(response.as_bytes()) {
            Ok(_) => debug!("OPTIONS response sent for path: {}", path),
            Err(e) => error!("failed sending OPTIONS response: {}", e),
        }
        match stream.flush() {
            Ok(_) => debug!("OPTIONS response flushed"),
            Err(e) => error!("failed flushing OPTIONS response: {}", e),
        }
    }

    pub fn json_response(mut stream: TcpStream, code: u16, data: String) {
        let response = format!(
            "HTTP/1.1 {code}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{data}\r\n"
        );
        match stream.write_all(response.as_bytes()) {
            Ok(_) => debug!("json response sent"),
            Err(e) => error!("failed sending json response: {}", e),
        }
        match stream.flush() {
            Ok(_) => debug!("response flushed"),
            Err(e) => error!("failed flushing json response: {}", e),
        }
    }

    pub fn handle_hello_world(mut stream: TcpStream) {
        let response = b"HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world budy!</body></html>\r\n";
        match stream.write_all(response) {
            Ok(_) => debug!("response sent"),
            Err(e) => error!("failed sending response: {}", e),
        }
        match stream.flush() {
            Ok(_) => debug!("response flushed"),
            Err(e) => error!("failed flushing json response: {}", e),
        }
    }

    pub fn handle_ping(mut stream: TcpStream) {
        let response =
            b"HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nPong!\r\n";
        match stream.write_all(response) {
            Ok(_) => debug!("response [ping] sent"),
            Err(e) => error!("failed sending response [ping]: {}", e),
        }
        match stream.flush() {
            Ok(_) => debug!("response [ping] flushed"),
            Err(e) => error!("failed flushing response [ping]: {}", e),
        }
    }

    pub fn handle_unknown_path(mut stream: TcpStream) {
        let response =
            b"HTTP/1.1 404\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nNot Found :(\r\n";
        match stream.write_all(response) {
            Ok(_) => debug!("response [unknown path] sent"),
            Err(e) => error!("failed sending response [unknown path]: {}", e),
        }
        match stream.flush() {
            Ok(_) => debug!("response flushed"),
            Err(e) => error!("failed flushing json response: {}", e),
        }
    }

    pub fn handle_method_not_allowed(mut stream: TcpStream, method: &str) {
        let message = format!("HTTP/1.1 405\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nMethod {} not allowed\r\n", method);
        let response = message.as_bytes();
        match stream.write_all(response) {
            Ok(_) => debug!("response [unknown path] sent"),
            Err(e) => error!("failed sending response [unknown path]: {}", e),
        }
        match stream.flush() {
            Ok(_) => debug!("response flushed"),
            Err(e) => error!("failed flushing json response: {}", e),
        }
    }

    pub fn handle_unauthorized(mut stream: TcpStream) {
        let message = b"HTTP/1.1 401 Unauthorized\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nUnauthorized\r\n";
        match stream.write_all(message) {
            Ok(_) => debug!("response [unauthorized] sent"),
            Err(e) => error!("failed sending response [unauthorized]: {}", e),
        }
        match stream.flush() {
            Ok(_) => debug!("response [unauthorized] flushed"),
            Err(e) => error!("failed flushing json [unauthorized] response: {}", e),
        }
    }
}
