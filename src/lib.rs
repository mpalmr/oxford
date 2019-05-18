//! HTTP client for the Oxford dictionary.

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

mod request;

use futures::Future;
use request::get_word_entries;

/// HTTP client containing all keys and other secrets.
pub struct Client {
    app_id: String,
    app_key: String,
    language: String,
}


impl Client {
    /// Create a new instance of Client.
    pub fn new(app_id: String, app_key: String, language: String) -> Self {
        Self {
            app_id,
            app_key,
            language,
        }
    }

    /// Looks up the definition of a provided word.
    pub fn lookup_word(&self, word: &str) -> impl Future<Item = (), Error = ()> {
        get_word_entries(&format!("{}/{}", self.base_url(), word))
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
