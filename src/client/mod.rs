extern crate serde;
extern crate reqwest;

use crate::models::*;
use crate::errors::{SabiError, Result};

pub struct Client {
    client: reqwest::Client
}

impl Client {
    pub fn new() -> Client {
        Client { client: reqwest::Client::new() }
    }

    pub async fn search_japanese_word(&self, word: String) -> Result<Vec<Word>> {
        let full_url = format!("https://jisho.org/api/v1/search/words?keyword={}", word);
        println!("{}", full_url);

        let resp = match self.client.get(&full_url).send().await {
            Ok(resp) => resp,
            Err(error) => return Err(SabiError::NetworkError(error))
        };
        let data: Data = match resp.json().await {
            Ok(data) => data,
            Err(error) => return Err(SabiError::NetworkError(error))
        };
        Ok(data.data)
    }

    pub async fn search_english_word(&self, mut word: String) -> Result<Vec<Word>> {
        word = format!("\"{}\"", word);
        self.search_japanese_word(word).await
    }
}