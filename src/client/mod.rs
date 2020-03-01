extern crate serde;
extern crate reqwest;

use crate::models::*;

pub struct Client {
    client: reqwest::Client
}

impl Client {
    pub fn new() -> Client {
        Client { client: reqwest::Client::new() }
    }

    pub async fn search_word(&self, word: String) -> Result<Word, reqwest::Error>{
        let full_url = format!("https://jisho.org/api/v1/search/words?keyword={}", word);

        let resp: Word = self.client.get(&full_url).send().await?.json().await?;
        Ok(resp)
    }
}