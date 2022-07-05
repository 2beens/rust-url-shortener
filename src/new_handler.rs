use http::StatusCode;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use redis::{Commands, Connection, RedisError};
use std::net::TcpStream;
use log::{debug, info};
use url::Url;
use urlencoding::decode;

extern crate redis;
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
        debug!("will add new url from post body: {}", post_body);

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

        info!("will be adding new url, raw: {}", url);
        let url = decode(url.as_str()).expect("UTF-8");
        info!("will be adding new url, decoded: {}", url);

        match Url::parse(&url) {
            Ok(parsed_url) => {
                debug!("new url is valid: {}", parsed_url.as_str());
            }
            Err(e) => {
                debug!("new url [{}] is NOT valid, err: {}", url, e);
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
        info!("new valid url {} will be linked and stored", new_id);

        let url_key = format!("short_url::{}", new_id);
        // TODO: in case error happens, unwrap() will panic; fix that, check for errors
        let _: () = self.redis_conn.set(&url_key, String::from(url.clone())).unwrap();
        let _: () = self.redis_conn.sadd("short_urls", url_key).unwrap();

        debug!("new url [{}] has been saved, path: /l/{}", url, new_id);
        Handlers::respond_with_status_code(stream, StatusCode::OK.as_u16(), format!("{}", new_id));
    }
}
