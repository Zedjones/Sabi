extern crate serde;

pub mod models {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct EnglishDefinition {
        #[serde(rename = "english_definitions")]
        definitions: Vec<String>,
        parts_of_speech: Vec<String>,
        tags: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct JapaneseWord {
        word: String,
        reading: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Word {
        #[serde(rename = "senses")]
        english_definitions: Vec<EnglishDefinition>,
        is_common: Option<bool>,
        tags: Vec<String>,
        #[serde(rename = "japanese")]
        japanese_words: Vec<JapaneseWord>,
    }
}
