#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(rust_2018_idioms)]
// #![warn(missing_docs)]

mod http;

use futures::Future;
use http::get_word_entries;

pub struct Client {
    app_id: String,
    app_key: String,
    language: String,
    client: reqwest::r#async::Client,
}

impl Client {
    pub fn new(app_id: String, app_key: String, language: String) -> Self {
        Self {
            app_id,
            app_key,
            language,
            client: reqwest::r#async::Client::new(),
        }
    }

    pub fn lookup_word(&self, word: &str) -> impl Future<Item = (), Error = ()> {
        let url = &format!("{}/{}", self.base_url(), word);
        get_word_entries(&self.client, url)
    }

    fn base_url(&self) -> String {
        format!(
            "https://od-api.oxforddictionaries.com/api/v2/{}",
            self.language
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn create_client() {
        let client = Client::new("abc".to_string(), "def".to_string(), "en-ca".to_string());
        assert_eq!(client.app_id, "abc".to_string());
        assert_eq!(client.app_key, "def".to_string());
        assert_eq!(client.language, "en-ca".to_string());
    }

    #[test]
    fn get_base_url() {
        let client = Client::new("abc".to_string(), "def".to_string(), "ghi".to_string());
        assert_eq!(
            client.base_url(),
            "https://od-api.oxforddictionaries.com/api/v2/ghi"
        );
    }
}
