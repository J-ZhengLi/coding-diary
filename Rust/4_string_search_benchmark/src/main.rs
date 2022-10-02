use aho_corasick::AhoCorasick;
use memchr::memmem;
use std::time::{Duration, Instant};

fn parse_duration(duration: Duration) -> String {
    let dur_micro = duration.as_micros();
    if dur_micro > 1_000_000 {
        format!("\x1b[31;1m{} seconds\x1b[0m", duration.as_secs_f32()) // red
    } else if dur_micro > 1_000 {
        format!("\x1b[33;1m{} milliseconds\x1b[0m", duration.as_millis()) // yellow
    } else {
        format!("\x1b[36;1m{} microseconds\x1b[0m", dur_micro) // cyan
    }
}

fn native_search(words: &[&str], context: &[u8], find_all: bool) {
    let str_context = std::str::from_utf8(context).unwrap();
    let timer = Instant::now();

    if words.len() == 1 {
        if find_all {
            println!(
                "[native] Found {} occurance of word \"{}\" in {}",
                str_context.match_indices(words[0]).count(),
                words[0],
                parse_duration(timer.elapsed())
            );
        } else {
            let result = str_context.find(words[0]);
            match result {
                Some(n) => {
                    println!(
                        "[native] Found word \"{}\" at position {} in {}",
                        words[0],
                        n,
                        parse_duration(timer.elapsed())
                    );
                }
                None => {
                    println!(
                        "[native] Did not find word \"{}\", time elapsed: {}",
                        words[0],
                        parse_duration(timer.elapsed())
                    );
                }
            }
        }
    } else {
        let mut counter: usize = 0;
        for &word in words {
            if find_all {
                counter += str_context.match_indices(word).count();
            } else {
                match str_context.find(word) {
                    Some(_) => {
                        counter += 1;
                        break;
                    }
                    None => {}
                }
            }
        }

        println!(
            "[native] Found {} occurance of words \"{:?}\" in {}",
            counter,
            words,
            parse_duration(timer.elapsed())
        );
    }
}

fn memmem_search(words: &[&str], context: &[u8], find_all: bool) {
    // start timer
    let timer = Instant::now();

    if words.len() == 1 {
        if find_all {
            let mem_it = memmem::find_iter(context, words[0]);
            println!(
                "[memmem] Found {} occurance of word \"{}\" in {}",
                mem_it.count(),
                words[0],
                parse_duration(timer.elapsed())
            );
        } else {
            let finder = memmem::Finder::new(words[0]);
            match finder.find(context) {
                Some(n) => {
                    println!(
                        "[memmem] Found word \"{}\" at position {} in {}",
                        words[0],
                        n,
                        parse_duration(timer.elapsed())
                    );
                }
                None => {
                    println!(
                        "[memmem] Did not find word \"{}\", time elapsed: {}",
                        words[0],
                        parse_duration(timer.elapsed())
                    );
                }
            }
        }
    } else {
        let mut counter: usize = 0;

        for &word in words {
            if find_all {
                counter += memmem::find_iter(context, word).count();
            } else {
                let finder = memmem::Finder::new(word);
                match finder.find(context) {
                    Some(_) => {
                        counter += 1;
                        break;
                    }
                    None => {}
                }
            }
        }

        println!(
            "[memmem] Found {} occurance of words \"{:?}\" in {}",
            counter,
            words,
            parse_duration(timer.elapsed())
        );
    }
}

fn aho_corasick_seach(words: &[&str], context: &[u8], find_all: bool) {
    let aho = AhoCorasick::new(words);

    // start timer
    let timer = Instant::now();

    if find_all {
        let result_count = aho.find_iter(context).count();
        println!(
            "[aho-corasick] Found {} occurance of words \"{:?}\" in {}",
            result_count,
            words,
            parse_duration(timer.elapsed())
        );
    } else {
        let found = aho.find(context);
        match found {
            Some(m) => {
                println!(
                    "[aho-corasick] Found leftmost match \"{}\" at position {} in {}",
                    std::str::from_utf8(&context[m.start()..m.end()]).unwrap(),
                    m.start(),
                    parse_duration(timer.elapsed())
                );
            }
            None => {
                println!(
                    "[aho-corasick] Did not find any word in list \"{:?}\", time elapsed: {}",
                    words,
                    parse_duration(timer.elapsed())
                );
            }
        }
    }
}

fn main() {
    // load text content which contains 1,000,000 words
    let full_text = include_bytes!("../res/longtext");

    // the first word being searched
    let word_1 = "B14fNx8YUy";
    // a non-existing word to be test
    let non_exist_word = "uwuwwwwxoxouuu";

    let three_words = &["tHGp3FEBx7", "skJCoFTN93", "fNx8YU"];

    let ten_words = &[
        "Hq6JYTKV",
        "7SC4yc4Vt",
        "58PjWm",
        "3tRoIEot",
        "97OUJh",
        "i6xiuKP",
        "fbkB0PB",
        "RUfmE4la",
        "H46v",
        "A_NON_EXISTING_WORD",
    ];

    println!(
        "\n==================== Finding non-existing string (worst-case) ===================="
    );
    native_search(&[non_exist_word], full_text, true);
    memmem_search(&[non_exist_word], full_text, true);
    aho_corasick_seach(&[non_exist_word], full_text, true);

    println!(
        "\n=================== Finding first occurrance of a single string ==================="
    );
    native_search(&[word_1], full_text, false);
    memmem_search(&[word_1], full_text, false);
    aho_corasick_seach(&[word_1], full_text, false);

    println!(
        "\n==================== Finding all occurrance of a single string ===================="
    );
    native_search(&[word_1], full_text, true);
    memmem_search(&[word_1], full_text, true);
    aho_corasick_seach(&[word_1], full_text, true);

    println!(
        "\n==================== Finding left most occurrance of three strings ===================="
    );
    native_search(three_words, full_text, false);
    memmem_search(three_words, full_text, false);
    aho_corasick_seach(three_words, full_text, false);

    println!("\n==================== Finding all occurrance of three strings ====================");
    native_search(three_words, full_text, true);
    memmem_search(three_words, full_text, true);
    aho_corasick_seach(three_words, full_text, true);

    println!(
        "\n==================== Finding left most occurrance of ten strings ===================="
    );
    native_search(ten_words, full_text, false);
    memmem_search(ten_words, full_text, false);
    aho_corasick_seach(ten_words, full_text, false);

    println!("\n==================== Finding all occurrance of ten strings ====================");
    native_search(ten_words, full_text, true);
    memmem_search(ten_words, full_text, true);
    aho_corasick_seach(ten_words, full_text, true);
}
