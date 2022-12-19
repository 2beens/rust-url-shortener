use http::StatusCode;
use log::{debug, info};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use redis::{Commands, Connection, RedisError};
use std::net::TcpStream;
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

        // TODO:
        // - add support for timestamp, so URLs can be ordered
        // - add support for custom URL ID
        // - protect sensitive endpoints (/new & /delete) with some auth
        //      - read session cookie and validate it
        // - validate custom_id - check if exists

        let (url, custom_id) = get_url_data_from_post_body(post_body);

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
        info!("new valid url, id [{}] will be linked and stored", new_id);

        let url_key = format!("short_url::{}", new_id);

        let id_inuse: bool = redis::cmd("SISMEMBER")
            .arg("short_urls")
            .arg(&url_key)
            .query(&mut self.redis_conn)
            .expect("failed to execute SISMEMBER for 'short_urls'");
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

        // TODO: in case error happens, unwrap() will panic; fix that, check for errors
        let _: () = self
            .redis_conn
            .set(&url_key, String::from(url.clone()))
            .unwrap();
        let _: () = self.redis_conn.sadd("short_urls", &url_key).unwrap();

        debug!("new url [{}] has been saved, path: /l/{}", url, new_id);
        Handlers::respond_with_status_code(stream, StatusCode::OK.as_u16(), format!("{}", new_id));
    }
}

// get_url_data_from_post_body returns found url and custom ID from thte post body
// - post_body expected form is: url=http://blabla&cid=some
fn get_url_data_from_post_body(post_body: String) -> (String, String) {
    let mut url = String::from("");
    let mut custom_id = String::from("");

    let post_body_parts: Vec<&str> = post_body.split_terminator("&").collect();
    if post_body_parts.len() == 0 {
        return (url, custom_id);
    }

    let first_param = post_body_parts[0];
    let first_param_parts: Vec<&str> = first_param.split_terminator("=").collect();
    match first_param_parts[0] {
        "url" => url = first_param_parts[1].to_string(),
        "cid" => custom_id = first_param_parts[1].to_string(),
        inv_param => debug!("invalid new link param: {}", inv_param),
    }

    if post_body_parts.len() < 2 {
        return (url, custom_id);
    }

    let second_param = post_body_parts[1];
    let second_param_parts: Vec<&str> = second_param.split_terminator("=").collect();
    match second_param_parts[0] {
        "url" => url = second_param_parts[1].to_string(),
        "cid" => custom_id = second_param_parts[1].to_string(),
        inv_param => debug!("invalid new link param: {}", inv_param),
    }

    (url, custom_id)
}

#[cfg(test)]
mod tests {
    use super::get_url_data_from_post_body;

    #[test]
    fn test_get_url_data_from_post_body() {
        let post_body = "url=http://2beens.xyz&cid=some".to_string();
        let (url, cid) = get_url_data_from_post_body(post_body);
        assert_eq!(url, "http://2beens.xyz");
        assert_eq!(cid, "some");

        let post_body = "cid=some&url=http://2beens.xyz".to_string();
        let (url, cid) = get_url_data_from_post_body(post_body);
        assert_eq!(url, "http://2beens.xyz");
        assert_eq!(cid, "some");

        let post_body = "url=http://2beens.xyz".to_string();
        let (url, cid) = get_url_data_from_post_body(post_body);
        assert_eq!(url, "http://2beens.xyz");
        assert_eq!(cid, "");

        let post_body = "cid=blabla".to_string();
        let (url, cid) = get_url_data_from_post_body(post_body);
        assert_eq!(url, "");
        assert_eq!(cid, "blabla");

        let post_body = "".to_string();
        let (url, cid) = get_url_data_from_post_body(post_body);
        assert_eq!(url, "");
        assert_eq!(cid, "");
    }
}
