use chrono::{prelude::*, Duration};
use log::debug;
use redis::{Commands, Connection};
extern crate chrono;
extern crate redis;

const SESSION_KEY_PREFIX: &str = "serj-service-session||";
const DEFAULT_TTL_DAYS: i64 = 7;

pub struct AuthService {
    redis_conn: Connection,
}

impl AuthService {
    pub fn new(redis_conn: Connection) -> AuthService {
        AuthService { redis_conn }
    }

    pub fn is_logged(&mut self, token: &String) -> bool {
        if token == "" {
            return false;
        }

        let session_key = format!("{}{}", SESSION_KEY_PREFIX, token);
        let created_at_unix_str: String = match self.redis_conn.get(session_key) {
            Ok(v) => v,
            Err(e) => {
                debug!("failed to find token in sessions [{}]: {}", token, e);
                return false;
            }
        };

        let created_at_unix = created_at_unix_str.parse::<i64>().unwrap();
        debug!(
            "auth service, checking logged in for [{}], created at: {}",
            token, created_at_unix
        );

        let created_at_naive = NaiveDateTime::from_timestamp_opt(created_at_unix, 0).unwrap();
        let created_at: DateTime<Utc> = DateTime::from_utc(created_at_naive, Utc);
        let created_at_with_ttl = created_at
            .checked_add_signed(Duration::days(DEFAULT_TTL_DAYS))
            .unwrap();

        if Utc::now() > created_at_with_ttl {
            return false;
        }

        return true;
    }
}
