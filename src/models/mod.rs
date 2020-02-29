extern crate serde;

pub mod models {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct EnglishDefinition {
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
        english_definitions: Vec<EnglishDefinition>,
        is_common: bool,
        tags: Vec<String>,
        japanese_words: Vec<JapaneseWord>,
    }
}
