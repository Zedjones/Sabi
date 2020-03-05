mod models;
mod client;
mod errors;

pub use models::Word;
pub use client::Client;
pub use errors::{Result, SabiError};

#[cfg(test)]
mod tests {
    use crate::{Client, errors::Result};

    use mockito::mock;
    fn create_ok_mock(path: &str, json: &str) -> mockito::Mock {
        mock("GET", &*format!("/{}", path))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json)
    }
    fn create_fail_mock(path: &str) -> mockito::Mock {
        mock("GET", &*format!("/{}", path))
            .with_status(404)
    }
    #[tokio::test]
    async fn empty_data() -> Result<()> {
        let term = "computer";
        let _m = create_ok_mock(term, r#"{ "data": [] }"#).create();
        assert!(Client::new().search_japanese_word(term).await?.is_empty());
        Ok(())
    }
    #[tokio::test]
    async fn network_error() -> Result<()> {
        let term = "computer";
        let _m = create_fail_mock(term).create();
        let res = Client::new().search_japanese_word(term).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("EOF while parsing a value"));
        Ok(())
    }
}
