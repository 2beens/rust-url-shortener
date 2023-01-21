use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct URLRecord {
    pub url: String,
    pub timestamp: i64,
}

impl URLRecord {
    pub fn from_json(json: &String) -> URLRecord {
        let rec: URLRecord = match serde_json::from_str(&json) {
            Ok(val) => val,
            Err(_) => {
                // backwards compatibility: ignore err, url is (most likely) from the previous model
                //  which contained only the url itself
                return URLRecord {
                    timestamp: 0,
                    url: json.to_string(),
                }
            }
        };
        return rec
    }

    pub fn to_json(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
