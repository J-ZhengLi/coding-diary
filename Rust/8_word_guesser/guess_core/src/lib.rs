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
            
            body
        }

        /// Use html parser when I have enough time to make one
        fn fetch_words_online() -> Vec<String> {
            let full_text = Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async { load_from_url(SOURCE_URL).await });
            
            let splitted = full_text.split("<li>");
            let mut result: Vec<String> = Vec::<String>::new();
            for s in splitted {
                if let Some((_, word_with_end_tags)) = s.trim().split_once("\">") {
                    if let Some((word, _)) = word_with_end_tags.trim().split_once("<") {
                        if word.len() > 0 {
                            result.push(word.trim().to_string());
                        }
                    }
                }
            }
            result.truncate(result.len() - 4);
            result
        }
    }
}

#[allow(unreachable_code)]
pub fn get_word_list() -> Vec<String> {
    #[cfg(feature = "web")]
    let result = fetch_words_online();

    #[cfg(not(feature = "web"))]
    let full_text: String = include_str!("../../res/text/insult_list.txt").to_string();
    #[cfg(not(feature = "web"))]
    let result = full_text.split(',').map(String::from).collect();

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
        println!("{:?}", result);
        assert!(!result.is_empty());
    }
}
