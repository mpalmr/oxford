use futures::{Future, Stream};
use reqwest::r#async::Client;
use serde::Deserialize;
use std::io::{self, Cursor};

pub fn get_word_entries(url: &str) -> impl Future<Item = (), Error = ()> {
    Client::new()
        .get(url)
        .send()
        .and_then(|res| res.into_body().concat2())
        .map_err(|err| eprintln!("Request error: {}", err))
        .map(|body| {
            let mut body = Cursor::new(body);
            if let Err(err) = io::copy(&mut body, &mut io::stdout()) {
                eprintln!("stdout error: {}", err);
            }
        })
}

#[derive(Deserialize)]
pub struct WordEntries {
    pub results: Vec<EntriesResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EntriesResult {
    pub id: String,
    pub language: String,
    pub lexical_entries: Vec<LexicalEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LexicalEntry {
    pub derivative_of: Vec<DerivativeOf>,
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
#[serde(rename_all = "snake_case")]
pub struct Entry {
    pub id: String,
    pub homograph_number: String,
    pub etymologies: Vec<String>,
    pub grammatical_features: Vec<TypedItem>,
    pub notes: Vec<TypedItem>,
    pub pronunciations: Vec<Pronunciation>,
    pub senses: Vec<Sense>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Pronunciation {
    pub audio_file: String,
    pub phonetic_notation: String,
    pub phonetic_spelling: String,
    pub dialects: Vec<String>,
    pub regions: Vec<Item>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Sense {
    pub definitions: Vec<String>,
    pub cross_reference_markers: Vec<String>,
    pub domains: Vec<Item>,
    pub cross_references: Vec<TypedItem>,
    pub examples: Vec<SenseExample>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SenseExample {
    pub text: String,
    pub definitions: Vec<String>,
    pub domains: Vec<String>,
    pub sense_ids: Vec<String>,
    pub regions: Vec<Item>,
    pub registers: Vec<Item>,
    pub notes: Vec<TypedItem>,
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
