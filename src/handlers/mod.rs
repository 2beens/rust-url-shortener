use http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::Write;
use std::net::TcpStream;
use url::Url;

extern crate redis;
use redis::Commands;

pub struct Handlers {}

impl Handlers {
    pub fn handle_link(stream: TcpStream, path: &str) {
        let url_id;
        match path.strip_prefix("/l/") {
            Some(url_id_from_path) => {
                url_id = String::from(url_id_from_path);
            },
            None => {
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::BAD_REQUEST.as_u16(),
                    String::from("url id param missing"),
                );
                return;
            },
        }

        // TODO: extract the redis client in a field or somewhere else
        let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut redis_conn = redis_client.get_connection().unwrap();
        let url_key = format!("short_url::{}", url_id);
        match redis_conn.get::<String, String>(url_key) {
            Ok(url) => {
                println!(">>> found url to redirect to: [{}]", url);
                Handlers::handle_redirect(stream, url);
            },
            Err(e) => {
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::BAD_REQUEST.as_u16(),
                    String::from(format!("redis err: {}", e)),
                );
            },
        }
    }

    pub fn handle_new(stream: TcpStream, post_body: String) {
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

        let new_id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        println!("new valid url {} will be linked and stored", new_id);

        // TODO: extract the redis client in a field or somewhere else
        let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut redis_conn = redis_client.get_connection().unwrap();

        let url_key = format!("short_url::{}", new_id);
        // throw away the result, just make sure it does not fail
        let _: () = redis_conn.set(&url_key, url.as_str()).unwrap();

        let message = format!("new url [{}] has been saved, path: /l/{}", url, new_id);
        println!("{}", message);
        Handlers::respond_with_status_code(stream, StatusCode::OK.as_u16(), message);
    }

    pub fn handle_redirect(mut stream: TcpStream, url: String) {
        let content =
r#"<html>
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

{content}"#);

        match stream.write_all(response.as_bytes()) {
            Ok(_) => println!("redirect response sent: {}", response),
            Err(e) => println!("failed sending redirect response: {}", e),
        }
        match stream.flush() {
            Ok(_) => println!("response [redirect] flushed"),
            Err(e) => println!("failed flushing response [redirect]: {}", e),
        }
    }

    pub fn respond_with_status_code(mut stream: TcpStream, code: u16, message: String) {
        let response = format!(
            "HTTP/1.1 {code}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{message}\r\n"
        );
        match stream.write_all(response.as_bytes()) {
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
