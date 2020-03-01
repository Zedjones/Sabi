mod models;
mod client;

#[cfg(test)]
mod tests {
    use crate::client::Client;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn search_client() {
        let client = Client::new();
        //client.search_word(String::from("iie")).await;
    }
}
