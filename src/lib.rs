#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]

use reqwest::{Client, Error};
use serde::Deserialize;

const BASE_URL: &str = "https://od-api.oxforddictionaries.com/api/v2";

pub struct Api {
    app_id: String,
    app_key: String,
    base_url: String,
}

impl Api {
    // TODO: enum for source_lang
    pub fn new(app_id: String, app_key: String, language: &str) -> Self {
        Self {
            app_id,
            app_key,
            base_url: format!("{}/{}", BASE_URL, language),
        }
    }

    pub fn lookup_word(&self, word_id: &str) -> Result<GetEntriesResponse, Error> {
        Ok(Client::new()
            .get(&format!("{}/{}", self.base_url, word_id))
            .send()?
            .json()?)
    }
}

#[derive(Deserialize)]
pub struct GetEntriesResponse {
    pub results: Vec<GetEntriesResponseResult>,
}

#[derive(Deserialize)]
pub struct GetEntriesResponseResult {
    pub id: String,
    pub language: String,
}

#[cfg(test)]
mod tests {
    use super::{BASE_URL, Api};

    #[test]
    fn new_api() {
        let api = Api::new("abc".to_string(), "def".to_string(), "en-ca");
        assert_eq!(api.app_id, "abc".to_string());
        assert_eq!(api.app_key, "def".to_string());
        assert_eq!(api.base_url, format!("{}/{}", BASE_URL, "en-ca"));
    }
}
