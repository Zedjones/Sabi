mod models;
mod client;
mod errors;

pub use models::{Word, EnglishDefinition};
pub use client::Client;
pub use errors::{Result, SabiError};

#[cfg(test)]
mod tests {
    use crate::client::Client;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[tokio::test]
    async fn search_client() {
        let client = Client::new();
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
