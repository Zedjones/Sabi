mod models;
mod client;
mod errors;

pub use models::{Word, JapaneseWord, EnglishDefinition};
pub use client::Client;
pub use errors::{Result, SabiError};

#[cfg(test)]
extern crate tokio;
extern crate reqwest;

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[tokio::test]
    async fn search_client() {
        let client = crate::client::Client::new();
        let res: crate::errors::Result<Vec<crate::models::Word>>;
        res = client.search_english_word(String::from("computer")).await;
        match res {
            Ok(words) => {
                println!("{:?}", words);
                println!("{}", words.len());
            },
            Err(err) => println!("{}", err)
        }
    }
}
