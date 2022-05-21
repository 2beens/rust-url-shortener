use http::StatusCode;
use std::io::Write;
use std::net::TcpStream;
use url::Url;

pub struct Handlers {}

impl Handlers {
    pub fn handle_link(mut stream: TcpStream) {
        // get original URL via link/ID from redis
        // redirect to original URL
    }

    pub fn handle_new(mut stream: TcpStream, post_body: String) {
        println!("will add new url: {}", post_body);

        let mut iter = post_body.split_terminator("=");
        if let Some(url_param) = iter.next() {
            if url_param != "url" {
                let err_message = format!("unexpected parameter: {}", url_param);
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::BAD_REQUEST.as_u16(),
                    err_message,
                );
                return;
            }
        }

        let mut url = String::from("");
        if let Some(found_url) = iter.next() {
            if found_url.len() == 0 {
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::BAD_REQUEST.as_u16(),
                    String::from("empty url parameter"),
                );
                return;
            }
            url = String::from(found_url);
        }

        println!("will be adding new url: {}", url);

        match Url::parse(&url) {
            Ok(parsed_url) => {
                println!("new url is valid: {}", parsed_url.as_str());
            }
            Err(e) => {
                println!("new url [{}] is NOT valid, err: {}", url, e);
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::BAD_REQUEST.as_u16(),
                    e.to_string(),
                );
                return;
            }
        }

        println!("new valid url will be linked and stored");

        // read url from the body
        // check URL is OK
        // generate ID
        // store in redis
    }

    pub fn respond_with_status_code(mut stream: TcpStream, code: u16, message: String) {
        let response = format!(
            "HTTP/1.1 {code}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{message}\r\n"
        );
        match stream.write(response.as_bytes()) {
            Ok(_) => println!("response sent"),
            Err(e) => println!("failed sending response: {}", e),
        }
    }

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
