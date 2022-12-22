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

        debug!(
            "auth service, checking logged in for [{}], created at: {}",
            token, created_at_unix_str
        );

        return is_created_at_valid(Utc::now(), created_at_unix_str);
    }
}

fn is_created_at_valid(now: DateTime<Utc>, created_at_unix_str: String) -> bool {
    if created_at_unix_str == "" {
        return false;
    }

    let created_at_unix = created_at_unix_str.parse::<i64>().unwrap();
    let created_at_naive = NaiveDateTime::from_timestamp_opt(created_at_unix, 0).unwrap();
    let created_at: DateTime<Utc> = DateTime::from_utc(created_at_naive, Utc);
    let created_at_with_ttl = created_at
        .checked_add_signed(Duration::days(DEFAULT_TTL_DAYS))
        .unwrap();

    if now > created_at_with_ttl {
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;

    use super::is_created_at_valid;

    #[test]
    fn test_is_created_at_valid() {
        let now: DateTime<Utc> = Utc.with_ymd_and_hms(2022, 12, 25, 0, 0, 0).unwrap();
        assert_eq!(true, is_created_at_valid(now, "1671731525".to_string())); // 1671731525 = 2022 dec 22
        assert_eq!(false, is_created_at_valid(now, "1669139064".to_string())); // 1669139064 = 22 nov 22
        assert_eq!(false, is_created_at_valid(now, "".to_string()));
    }
}
