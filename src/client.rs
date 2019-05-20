extern crate reqwest;
extern crate serde_json;

use serde_json::json;

pub struct ConsulClient {
    pub host: String
}

impl ConsulClient {
    pub fn list(&self) -> serde_json::Value {
        let mut path = self.host.clone();
        path.push_str("/v1/kv/?keys&separator=%2F");

        let data: serde_json::Value = match reqwest::get(path.as_str()) {
            Ok(mut response) => match response.json() {
                Ok(json) => json,
                Err(e) => panic!(e)
            },
            Err(e) => panic!(e)
        };

        return data;
    }

    pub fn get(&self, subpath: String) -> String {
        let mut path = self.host.clone();
        path.push_str("/v1/kv/");
        path.push_str(subpath.as_str());
        if subpath.ends_with("/") {
            path.push_str("?keys&separator=%2F")
        } else {
            path.push_str("?raw")
        }

        let data: String = match reqwest::get(path.as_str()) {
            Ok(mut response) => match response.text() {
                Ok(json) => json,
                Err(e) => panic!(e)
            },
            Err(e) => panic!(e)
        };

        return data;
    }

    pub fn put(&self, key: String, body: String) -> bool {
        let mut path = self.host.clone();
        path.push_str("/v1/kv/");
        path.push_str(key.as_str());

        let client = reqwest::Client::new();

        let data: serde_json::Value = match client.put(path.as_str()).body(body).send() {
            Ok(mut response) => match response.json() {
                Ok(json) => json,
                Err(e) => panic!(e)
            },
            Err(e) => panic!(e)
        };
        return data.as_bool().unwrap();
    }
}
