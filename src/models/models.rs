extern crate serde;

use serde::{Deserialize, Serialize};

pub mod models {
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
