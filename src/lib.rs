#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod request;

pub struct Client {
    app_id: String,
    app_key: String,
    language: String,
}

impl Client {
    // TODO: enum for source_lang
    pub fn new(app_id: String, app_key: String, language: String) -> Self {
        Self {
            app_id,
            app_key,
            language,
        }
    }

    pub fn lookup_word(&self, word: &str) -> Result<request::get_word::Entries, reqwest::Error> {
        Ok(request::get_word::fetch(word)?)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn new_api() {
        let api = Client::new("abc".to_string(), "def".to_string(), "en-ca".to_string());
        assert_eq!(api.app_id, "abc".to_string());
        assert_eq!(api.app_key, "def".to_string());
        assert_eq!(api.language, "en-ca".to_string());
    }
}
