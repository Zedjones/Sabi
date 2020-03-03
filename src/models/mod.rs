extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EnglishDefinition {
    #[serde(rename = "english_definitions")]
    pub definitions: Vec<String>,
    pub parts_of_speech: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JapaneseWord {
    pub word: Option<String>,
    pub reading: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub data: Vec<Word>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "data")]
pub struct Word {
    #[serde(rename = "senses")]
    pub english_definitions: Vec<EnglishDefinition>,
    pub is_common: Option<bool>,
    pub tags: Vec<String>,
    #[serde(rename = "japanese")]
    pub japanese_words: Vec<JapaneseWord>,
}