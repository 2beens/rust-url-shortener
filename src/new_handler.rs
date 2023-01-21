use chrono::Utc;
use http::StatusCode;
use log::{debug, info};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use redis::{Commands, Connection, RedisError};
use std::net::TcpStream;
use url::Url;
use urlencoding::decode;

extern crate redis;
use crate::{handlers::Handlers, url_record::URLRecord};

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

        let (url, custom_id) = match get_url_data_from_post_body(post_body) {
            Ok((url, cid)) => (url, cid),
            Err(err) => {
                debug!("new url: {}", err);
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::BAD_REQUEST.as_u16(),
                    "invalid url".to_string(),
                );
                return;
            }
        };

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

        let new_id: String;
        if custom_id != "" {
            new_id = custom_id
        } else {
            new_id = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect();
        }
        info!("new valid url, id [{}] will be linked and stored", &new_id);

        let url_key = format!("short_url::{}", &new_id);

        let id_inuse: bool = match redis::cmd("SISMEMBER")
            .arg("short_urls")
            .arg(&url_key)
            .query(&mut self.redis_conn)
        {
            Ok(val) => val,
            Err(err) => {
                debug!("failed to execute SISMEMBER for 'short_urls': {}", err);
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    err.to_string(),
                );
                return;
            }
        };
        if id_inuse {
            debug!(
                "error, url with key {} already exists, skipping add",
                new_id
            );
            Handlers::respond_with_status_code(
                stream,
                StatusCode::BAD_REQUEST.as_u16(),
                "already exists".to_string(),
            );
            return;
        }

        let url_record = URLRecord {
            id: new_id.to_string(),
            url: url.to_string(),
            timestamp: Utc::now().timestamp(),
            hits: 0,
        };

        let url_record_json = url_record.to_json();
        println!("++ storing new url record: {}", url_record_json);

        let _: () = match self.redis_conn.set(&url_key, String::from(url_record_json)) {
            Ok(val) => val,
            Err(err) => {
                debug!(
                    "failed to execute SET for new url key [{}]: {}",
                    url_key, err
                );
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    err.to_string(),
                );
                return;
            }
        };

        let _: () = match self.redis_conn.sadd("short_urls", &url_key) {
            Ok(val) => val,
            Err(err) => {
                debug!("failed to execute SADD for 'short_urls': {}", err);
                Handlers::respond_with_status_code(
                    stream,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    err.to_string(),
                );
                return;
            }
        };

        debug!("new url [{}] has been saved, path: /l/{}", url, new_id);
        Handlers::respond_with_status_code(stream, StatusCode::OK.as_u16(), format!("{}", new_id));
    }
}

// get_url_data_from_post_body returns found url and custom ID from thte post body
// - post_body expected form is: url=http://blabla&cid=some
fn get_url_data_from_post_body(post_body: String) -> Result<(String, String), String> {
    let mut url = String::from("");
    let mut custom_id = String::from("");

    let post_body_parts: Vec<&str> = post_body.split_terminator("&").collect();
    if post_body_parts.len() == 0 {
        return Err("post body invalid (0 parts)".to_string());
    }

    let first_param = post_body_parts[0];
    let first_param_parts: Vec<&str> = first_param.split_terminator("=").collect();
    if first_param_parts.len() != 2 {
        return Err(format!(
            "invalid parameter: {}, no value found",
            first_param_parts[0]
        ));
    }
    match first_param_parts[0] {
        "url" => url = first_param_parts[1].to_string(),
        "cid" => custom_id = first_param_parts[1].to_string(),
        inv_param => debug!("invalid new link param: {}", inv_param),
    }

    if post_body_parts.len() < 2 {
        if url == "" {
            return Err("url param not found".to_string());
        }
        return Ok((url, custom_id));
    }

    let second_param = post_body_parts[1];
    let second_param_parts: Vec<&str> = second_param.split_terminator("=").collect();
    if second_param_parts.len() != 2 {
        return Err(format!(
            "invalid parameter: {}, no value found",
            second_param_parts[0]
        ));
    }
    match second_param_parts[0] {
        "url" => url = second_param_parts[1].to_string(),
        "cid" => custom_id = second_param_parts[1].to_string(),
        inv_param => debug!("invalid new link param: {}", inv_param),
    }

    return Ok((url, custom_id));
}

#[cfg(test)]
mod tests {
    use super::get_url_data_from_post_body;

    fn test_get_url_data_case(
        post_body: &str,
        want_url: &str,
        want_cid: &str,
    ) -> Result<(), String> {
        let (url, cid) = match get_url_data_from_post_body(post_body.to_string()) {
            Ok((url, cid)) => (url, cid),
            Err(err) => {
                return Err(err);
            }
        };
        if url != want_url {
            return Err(format!("want url: {}, but got: {}", want_url, url));
        }
        if cid != want_cid {
            return Err(format!("want cid: {}, but got: {}", want_cid, cid));
        }
        Ok(())
    }

    #[test]
    fn test_get_url_data_from_post_body_valid_cases() -> Result<(), String> {
        [
            (
                "url=http://2beens.xyz&cid=some",
                "http://2beens.xyz",
                "some",
            ),
            (
                "cid=some&url=http://2beens.xyz",
                "http://2beens.xyz",
                "some",
            ),
            ("url=http://2beens.xyz", "http://2beens.xyz", ""),
        ]
        .iter()
        .try_for_each(|(pb, url, cid)| test_get_url_data_case(*pb, *url, *cid))?;

        Ok(())
    }

    #[test]
    fn test_get_url_data_from_post_body_invalid_cases() -> Result<(), String> {
        [
            "",
            "blabla",
            "url=&cid=some",
            "cid=&url=some",
            "url==",
            "url==",
            "cid=id1",
        ]
        .iter()
        .try_for_each(|pb| {
            let (url, cid) = match get_url_data_from_post_body((*pb).to_string()) {
                Ok((url, cid)) => (url, cid),
                Err(_) => return Ok(()),
            };
            Err(format!(
                "unexpected url and cid received for [{}]: {}, {}",
                pb, url, cid
            ))
        })?;

        Ok(())
    }
}
