use crate::models::*;
use crate::errors::{SabiError, Result};
use cfg_if::cfg_if;

pub struct Client {
    client: reqwest::Client
}

impl Client {
    pub fn new() -> Client {
        Client { client: reqwest::Client::new() }
    }

    pub async fn search_japanese_word(&self, word: String) -> Result<Vec<Word>> {
        cfg_if! {
            if #[cfg(test)] {
                let full_url = mockito::server_url();
                // Avoid unused variables error
                let _ = word;
            }
            else {
                let full_url = format!("https://jisho.org/api/v1/search/words?keyword={}", word);
            }
        }

        let resp = match self.client.get(&full_url).send().await {
            Ok(resp) => resp,
            Err(error) => return Err(SabiError::new(error))
        };
        let data: Data = match resp.json().await {
            Ok(data) => data,
            Err(error) => return Err(SabiError::new(error))
        };
        Ok(data.data)
    }

    pub async fn search_english_word(&self, word: String) -> Result<Vec<Word>> {
        self.search_japanese_word(format!("\"{}\"", word)).await
    }
}