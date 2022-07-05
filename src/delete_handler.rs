use http::StatusCode;
use redis::{Commands, Connection, RedisError};
use std::net::TcpStream;
use log::debug;

extern crate redis;
use crate::handlers::Handlers;

pub struct DeleteHandler {
    redis_conn: Connection,
}

impl DeleteHandler {
    pub fn new(redis_conn_string: &String) -> Result<DeleteHandler, RedisError> {
        let redis_client = redis::Client::open(String::from(redis_conn_string))?;
        let redis_conn = redis_client.get_connection()?;
        Ok(DeleteHandler { redis_conn })
    }

    pub fn handle_delete(&mut self, stream: TcpStream, path: &str) {
        debug!("will delete url: {}", path);

        let path_parts =  path.split("?");
        let path_parts_vec = path_parts.collect::<Vec<&str>>();
        if path_parts_vec.len() != 2 {
            Handlers::respond_with_status_code(stream, StatusCode::BAD_REQUEST.as_u16(), String::from("request path seems fucked up"));
            return
        }

        let id_parts = path_parts_vec[1].split("=");
        let id_parts_vec = id_parts.collect::<Vec<&str>>();
        if id_parts_vec.len() != 2 || id_parts_vec[0] != "id" {
            Handlers::respond_with_status_code(stream, StatusCode::BAD_REQUEST.as_u16(), String::from("invalid url id info"));
            return
        }

        let id = id_parts_vec[1];
        if id == "" {
            Handlers::respond_with_status_code(stream, StatusCode::BAD_REQUEST.as_u16(), String::from("missing url id info"));
            return
        }

        debug!(">>> will be deleting url: {}", id);
        let url_key = format!("short_url::{}", id);
        let del_res: i32 = self.redis_conn.del(&url_key).expect("failed to delete url by key");

        let log_msg = format!("delete [{}] result: {}", id, del_res);
        debug!(">>> {}", log_msg);

        if del_res == 0 {
            Handlers::respond_with_status_code(stream, StatusCode::NOT_FOUND.as_u16(), String::from(log_msg));
            return;
        }

        // now remove the key from the short_urls set
        let del_res: i32 = self.redis_conn.srem("short_urls", &url_key).
            expect("failed to delete url key [{}] from the short_urls set");
        debug!("delete {} from short_urls set result: {}", url_key, del_res);

        Handlers::respond_with_status_code(stream, StatusCode::OK.as_u16(), String::from(log_msg));
    }
}
