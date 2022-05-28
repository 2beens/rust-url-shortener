extern crate redis;
use std::net::TcpStream;

use http::StatusCode;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use redis::{Commands, Connection, RedisError};
use url::Url;

use crate::handlers::Handlers;

pub struct NewHandler {
    redis_conn: Connection,
}

impl NewHandler {
    pub fn new(redis_conn_string: &String) -> Result<NewHandler, RedisError> {
        let redis_client = redis::Client::open(String::from(redis_conn_string))?;
        let redis_conn = redis_client.get_connection()?;
        Ok(NewHandler { redis_conn })
    }

    pub fn handle_new(&mut self, stream: TcpStream, post_body: String) {
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

        let url_key = format!("short_url::{}", new_id);
        // throw away the result, just make sure it does not fail
        let _: () = self.redis_conn.set(&url_key, url.as_str()).unwrap();

        let message = format!("new url [{}] has been saved, path: /l/{}", url, new_id);
        println!("{}", message);
        Handlers::respond_with_status_code(stream, StatusCode::OK.as_u16(), message);
    }
}
