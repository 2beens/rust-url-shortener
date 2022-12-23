use chrono::{prelude::*, Duration};
use log::debug;
use redis::{Commands, Connection};
extern crate chrono;
extern crate redis;

const SESSION_KEY_PREFIX: &str = "serj-service-session||";
const DEFAULT_TTL_DAYS: i64 = 7;

pub trait SessionStorage {
    fn get_session_created_at(&mut self, session_token: String) -> String;
}

impl SessionStorage for Connection {
    fn get_session_created_at(&mut self, session_token: String) -> String {
        let created_at_unix_str: String = match self.get(&session_token) {
            Ok(v) => v,
            Err(e) => {
                debug!(
                    "failed to find token in sessions [{}]: {}",
                    &session_token, e
                );
                return "".to_string();
            }
        };
        return created_at_unix_str;
    }
}

pub struct AuthService {
    session_storage: dyn SessionStorage,
}

impl AuthService {
    pub fn new(session_storage: dyn SessionStorage) -> AuthService {
        AuthService { session_storage }
    }

    pub fn is_logged(&mut self, token: &String) -> bool {
        if token == "" {
            return false;
        }

        let session_key = format!("{}{}", SESSION_KEY_PREFIX, token);
        let created_at_unix_str: String = self.session_storage.get_session_created_at(session_key);
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
