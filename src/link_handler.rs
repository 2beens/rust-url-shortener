use crate::{handlers::Handlers, url_record::URLRecord};
use http::StatusCode;
use log::debug;
use std::net::TcpStream;

extern crate redis;
use redis::{Commands, Connection, RedisError};

pub struct LinkHandler {
    redis_conn: Connection,
}

impl LinkHandler {
    pub fn new(redis_conn_string: &String) -> Result<LinkHandler, RedisError> {
        let redis_client = redis::Client::open(String::from(redis_conn_string))?;
        let redis_conn = redis_client.get_connection()?;
        Ok(LinkHandler { redis_conn })
    }

    pub fn handle_link(&mut self, stream: TcpStream, path: &str) {
        let url_id;
        match path.strip_prefix("/l/") {
            Some(url_id_from_path) => {
                url_id = String::from(url_id_from_path);
            }
            None => {
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::BAD_REQUEST.as_u16(),
                    String::from("url id param missing"),
                );
                return;
            }
        }

        debug!(">>> will redirect to url id: [{}]", url_id);

        let url_key = format!("short_url::{}", url_id);
        match self.redis_conn.get::<String, String>(url_key) {
            Ok(url_record) => {
                let url_record = URLRecord::from_json(url_id, &url_record);
                debug!(">>> found url to redirect to: [{}]", url_record.url);
                Handlers::handle_redirect(stream, url_record.url);
            }
            Err(e) => {
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::BAD_REQUEST.as_u16(),
                    String::from(format!("redis err: {}", e)),
                );
            }
        }
    }
}
