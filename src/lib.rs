mod models;
mod client;
mod errors;

pub use models::Word;
pub use client::Client;
pub use errors::{Result, SabiError};

#[cfg(test)]
mod tests {
    use crate::Client;
    use crate::Result;
    use crate::Word;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[tokio::test]
    async fn search_client() {
        let client = Client::new();
        let res: Result<Vec<Word>> = client.search_english_word(String::from("computer")).await;
        match res {
            Ok(words) => {
                println!("{:?}", words);
                println!("{}", words.len());
            },
            Err(err) => println!("{}", err)
        }
    }
}
