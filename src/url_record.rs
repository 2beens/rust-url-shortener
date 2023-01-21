use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct URLRecord {
    pub id: String,
    pub url: String,
    pub timestamp: i64,
}

impl URLRecord {
    // from_json will try go unmarshal, but for backwards compatility, this funciton also
    //  needs the original id for backfill... not nice
    pub fn from_json(id: String, json: &String) -> URLRecord {
        let rec: URLRecord = match serde_json::from_str(&json) {
            Ok(val) => val,
            Err(_) => {
                // backwards compatibility: ignore err, url is (most likely) from the previous model
                //  which contained only the url itself
                return URLRecord {
                    id: id,
                    timestamp: 0,
                    url: json.to_string(),
                };
            }
        };
        return rec;
    }

    pub fn to_json(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
