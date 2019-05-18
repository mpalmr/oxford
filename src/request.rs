use reqwest::Client;

const BASE_URL: &str = "https://od-api.oxforddictionaries.com/api/v2";

pub mod get_word {
    use serde::Deserialize;

    pub fn fetch(word: &str) -> Result<Entries, reqwest::Error> {
        let entries = super::Client::new()
            .get(&format!("{}/{}", super::BASE_URL, word))
            .send()?
            .json()?;
        Ok(entries)
    }

    #[derive(Deserialize)]
    pub struct Entries {
        pub results: Vec<EntriesResult>,
    }

    #[derive(Deserialize)]
    pub struct Item {
        pub id: String,
        pub text: String,
    }

    #[derive(Deserialize)]
    pub struct TypedItem {
        pub id: String,
        pub text: String,
        pub r#type: String,
    }

    #[derive(Deserialize)]
    pub struct EntriesResult {
        pub id: String,
        pub language: String,
        pub lexicalEntries: Vec<LexicalEntry>,
    }

    #[derive(Deserialize)]
    pub struct LexicalEntry {
        pub derivativeOf: Vec<DerivativeOf>,
        pub derivatives: Vec<Derivative>,
        pub entries: Vec<Entry>,
    }

    #[derive(Deserialize)]
    pub struct DerivativeOf {
        pub id: String,
        pub text: String,
        pub language: String,
        pub domains: Vec<Item>,
        pub regions: Vec<Item>,
    }

    #[derive(Deserialize)]
    pub struct Derivative {
        pub id: String,
        pub text: String,
        pub language: String,
        pub regions: Vec<Item>,
        pub domains: Vec<Item>,
        pub registers: Vec<Item>,
    }

    #[derive(Deserialize)]
    pub struct Entry {
        pub id: String,
        pub homographNumber: String,
        pub etymologies: Vec<String>,
        pub grammaticalFeatures: Vec<TypedItem>,
        pub notes: Vec<TypedItem>,
        pub pronunciations: Vec<Pronunciation>,
        pub senses: Vec<Sense>,
    }

    #[derive(Deserialize)]
    pub struct Pronunciation {
        pub audioFile: String,
        pub phoneticNotation: String,
        pub phoneticSpelling: String,
        pub dialects: Vec<String>,
        pub regions: Vec<Item>,
    }

    #[derive(Deserialize)]
    pub struct Sense {
        pub definitions: Vec<String>,
        pub crossReferenceMarkers: Vec<String>,
        pub domains: Vec<Item>,
        pub crossReferences: Vec<TypedItem>,
        pub examples: Vec<SenseExample>,
    }

    #[derive(Deserialize)]
    pub struct SenseExample {
        pub text: String,
        pub definitions: Vec<String>,
        pub domains: Vec<String>,
        pub senseIds: Vec<String>,
        pub regions: Vec<Item>,
        pub registers: Vec<Item>,
        pub notes: Vec<TypedItem>,
    }
}
