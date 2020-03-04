mod models;
mod client;
mod errors;

pub use models::Word;
pub use client::Client;
pub use errors::{Result, SabiError};

#[cfg(test)]
mod tests {
    use crate::Client;
    use crate::errors::Result;

    use mockito::mock;
    fn create_200_mock(path: &str, json: &str) -> mockito::Mock {
        mock("GET", &*format!("/{}", path))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json)
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[tokio::test]
    async fn empty_data() -> Result<()> {
        let term = "computer";
        let mock = create_200_mock(term, r#"{ "data": [] }"#);
        let _m = mock.create();
        assert!(Client::new().search_japanese_word(term).await?.is_empty());
        Ok(())
    }
    #[test]
    fn it_works_two() {
        assert_eq!(2 + 2, 4);
    }
}
