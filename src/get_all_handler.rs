use http::StatusCode;
use redis::{Commands, Connection, RedisError};
use std::{collections::HashSet, net::TcpStream};

extern crate redis;

use crate::handlers::Handlers;

pub struct GetAllHandler {
    redis_conn: Connection,
}

impl GetAllHandler {
    pub fn new(redis_conn_string: &String) -> Result<GetAllHandler, RedisError> {
        let redis_client = redis::Client::open(String::from(redis_conn_string))?;
        let redis_conn = redis_client.get_connection()?;
        Ok(GetAllHandler { redis_conn })
    }

    pub fn handle_get_all(&mut self, stream: TcpStream) {
        println!("trying to find and return all links ...");

        let mut res_json: Vec<String> = vec![String::from("[")];

        // get all link ids
        let url_keys: HashSet<String> = self.redis_conn.smembers("short_urls").unwrap();
        for url_key in &url_keys {
            match self.redis_conn.get::<&String, String>(&url_key) {
                Ok(url) => {
                    res_json.push(format!(r#"{{ "key": "{}", "url": "{}" }}"#, &url_key, url));
                    res_json.push(String::from(","))
                }
                Err(e) => {
                    println!("error reading URL by key [{}]: {}", &url_key, e)
                }
            }
        }

        // pop last comma
        if &url_keys.len() > &0 {
            res_json.pop();
        }

        res_json.push(String::from("]"));

        Handlers::json_response(stream, StatusCode::OK.as_u16(), res_json.concat());
    }
}
