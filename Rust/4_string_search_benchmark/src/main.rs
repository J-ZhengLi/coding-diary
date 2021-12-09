use std::time::Instant;
use memchr::memmem;
use aho_corasick::AhoCorasick;

fn native_search(word: &str, context: &[u8], find_all: bool) {
    let str_context = std::str::from_utf8(context).unwrap();

    let timer = Instant::now();
    if find_all {
        let result: Vec<_> = str_context.match_indices(word).collect();
        println!("[native] Found {} occurance of word \"{}\" in {} microsecs", result.len(), word, timer.elapsed().as_micros());
    } else {
        let result = str_context.find(word);
        match result {
            Some(n) => {
                println!("[native] Found word \"{}\" at position {} in {} microsecs",
                        word,
                        n,
                        timer.elapsed().as_micros());
            },
            None => {
                println!("[native] Did not find word \"{}\", time elapsed: {} microsecs",
                        word,
                        timer.elapsed().as_micros());
            }
        }
    }
}

fn memmem_search(word: &str, context: &[u8], find_all: bool) {
    // start timer
    let timer = Instant::now();

    if find_all {
        let mem_it = memmem::find_iter(context, word);
        println!("[memmem] Found {} occurance of word \"{}\" in {} microsecs", mem_it.count(), word, timer.elapsed().as_micros());
    } else {
        let finder = memmem::Finder::new(word);
        match finder.find(context) {
            Some(n) => {
                println!("[memmem] Found word \"{}\" at position {} in {} microsecs",
                        word,
                        n,
                        timer.elapsed().as_micros());
            },
            None => {
                println!("[memmem] Did not find word \"{}\", time elapsed: {} microsecs",
                        word,
                        timer.elapsed().as_micros());
            }
        }
    }
}

fn aho_corasick_seach(words: &[&str], context: &[u8], find_all: bool) {
    let aho = AhoCorasick::new(words);

    // start timer
    let timer = Instant::now();

    if find_all {
        let result_count = aho.find_iter(context).count();
        println!("[aho-corasick] Found {} occurance of words \"{:?}\" in {} microsecs", result_count, words, timer.elapsed().as_micros());
    } else {
        let found = aho.find(context);
        match found {
            Some(m) => {
                println!("[aho-corasick] Found leftmost match \"{}\" at position {} in {} microsecs",
                        std::str::from_utf8(&context[m.start()..m.end()]).unwrap(),
                        m.start(),
                        timer.elapsed().as_micros());
            },
            None => {
                println!("[aho-corasick] Did not find any word in list \"{:?}\", time elapsed: {} microsecs",
                        words,
                        timer.elapsed().as_micros());
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

    println!("\n==================== Finding non-existing string (worst-case) ====================");
    native_search(non_exist_word, full_text, true);
    memmem_search(non_exist_word, full_text, true);
    aho_corasick_seach(&[non_exist_word], full_text, true);

    println!("\n=================== Finding first occurrance of a single string ===================");
    native_search(word_1, full_text, false);
    memmem_search(word_1, full_text, false);
    aho_corasick_seach(&[word_1], full_text, false);

    println!("\n==================== Finding all occurrance of a single string ====================");
    native_search(word_1, full_text, true);
    memmem_search(word_1, full_text, true);
    aho_corasick_seach(&[word_1], full_text, true);
}
