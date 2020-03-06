mod models;
mod client;
mod errors;

pub use models::Word;
pub use client::Client;
pub use errors::{Result, SabiError};

#[cfg(test)]
mod tests {
    use crate::{Client, errors::Result, models::Word};
    use std::{fs::read_to_string, path::PathBuf};

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
        assert!(res.unwrap_err().to_string().contains("EOF while parsing a value"));
        Ok(())
    }
    #[tokio::test]
    async fn basic_word() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/test/test.json");
        let term = "unten";
        let json = read_to_string(path).unwrap();
        let _m = create_ok_mock(term, &json).create();
        let res: Vec<Word> = Client::new().search_japanese_word(term).await?;
        let fst_item = res.first().expect("no first word");
        assert!(fst_item
                .english_definitions
                .first()
                .expect("no first element in english definitionss")
                .definitions
                .contains(&"operating".to_string()));
        let fst_jp_word = fst_item.japanese_words.first().expect("no first japanese definition");
        assert_eq!(fst_jp_word.word, Some("運転".to_string()));
        assert_eq!(fst_jp_word.reading, Some("うんてん".to_string()));
        assert_eq!(fst_item.is_common, Some(true));
        Ok(())
    }
}
