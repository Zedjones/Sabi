extern crate serde;
extern crate reqwest;

pub mod client {
    use crate::models::models::*;

    pub async fn search_word(word: String) {
        let full_url = format!("https://jisho.org/api/v1/search/words?keyword={}", word);

        let resp = reqwest::get(&full_url).await;
    }

}