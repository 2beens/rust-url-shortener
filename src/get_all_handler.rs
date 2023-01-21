use http::StatusCode;
use log::{debug, warn};
use redis::{Commands, Connection, RedisError};
use std::{collections::HashSet, net::TcpStream};

extern crate redis;

use crate::{handlers::Handlers, url_record::URLRecord};

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
        debug!("trying to find and return all links ...");

        // get all link ids
        let url_keys: HashSet<String> = match self.redis_conn.smembers("short_urls") {
            Ok(uk) => uk,
            Err(err) => {
                debug!("failed to execute SMEMBERS for 'short_urls': {}", err);
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    err.to_string(),
                );
                return;
            }
        };

        let mut url_records = vec![];

        for url_key in &url_keys {
            match self.redis_conn.get::<&String, String>(&url_key) {
                Ok(url_record) => {
                    // url key is created as: format!("short_url::{}", new_id);
                    let url_id = url_key.split("::");
                    let url_id = url_id.collect::<Vec<&str>>();
                    if url_id.len() != 2 {
                        warn!("!! invalid url key: {}", url_key);
                        continue;
                    }
                    let url_id = url_id[1];

                    let url_record = URLRecord::from_json(url_id.to_string(), &url_record);
                    url_records.push(url_record);
                }
                Err(e) => {
                    debug!("error reading URL by key [{}]: {}", &url_key, e)
                }
            }
        }

        let res_json = serde_json::to_string(&url_records).unwrap();

        Handlers::json_response(stream, StatusCode::OK.as_u16(), res_json);
    }
}
