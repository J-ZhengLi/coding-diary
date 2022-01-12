cfg_if::cfg_if! {
    if #[cfg(feature = "web")] {
        use reqwest;
        use tokio::runtime::Builder;

        const SOURCE_URL: &str = "https://www.insult.wiki/list-of-insults";

        async fn load_from_url(url: &str) -> String {
            let body = reqwest::get(url)
                .await.expect("Could not connect to given url.")
                .text()
                .await.expect("Fail to get the full response text.");

            println!("Body: {}", body);
            String::new()
        }
    }
}

pub fn get_word_list() -> Vec<String> {
    #[cfg(feature = "web")]
    let full_text = Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { load_from_url(SOURCE_URL).await });

    #[cfg(not(feature = "web"))]
    let full_text: String = include_str!("../../res/text/insult_list").to_string();

    let result: Vec<String> = Vec::from_iter(full_text.split_whitespace().map(String::from));

    result
}

#[cfg(test)]
mod tests {
    use crate::get_word_list;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn fetch_raw_html() {
        let result = get_word_list();
        assert!(!result.is_empty(), "Result: {:?}", result);
    }
}
