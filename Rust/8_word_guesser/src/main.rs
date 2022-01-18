//! # Word Guesser
//!
//! In order to guess word as user type,
//! the program needs to have 3 threads.
//! One for the main thread, which shows input and output,
//! one for reading user input,
//! and one for processing the guesses.

use guess_core::get_word_list;
use std::fmt::Display;
use std::io::{self, stdin, Read, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::{thread, time::Duration};
use termion::{
    cursor::{self, DetectCursorPos},
    raw::IntoRawMode,
};
use strsim::normalized_levenshtein;
use once_cell::sync::OnceCell;

fn add_char(mutex_s: &Mutex<String>, ch: char) {
    let mut old_s = mutex_s
        .lock()
        .expect("Something went wrong when adding character to mutex string.");

    (*old_s).push(ch);
}

fn pop_char(mutex_s: &Mutex<String>) {
    let mut old_s = mutex_s
        .lock()
        .expect("Something went wrong when removing character from a mutex string.");

    (*old_s).pop();
}

static WORDS_CELL: OnceCell<Vec<String>> = OnceCell::new();

fn find_similar_word_in_list(word: &str, list: &Vec<String>) -> Option<String> {
    let is_single_char: bool = word.len() == 1;
    let mut max_score = 0_f64;
    let mut str_with_max_score: String = String::new();
    for candidate in list {
        if is_single_char && candidate.starts_with(word) {
            return Some(candidate.to_string());
        }
        let score = normalized_levenshtein(word, candidate);
        if score > max_score {
            max_score = score;
            str_with_max_score = candidate.to_string();
        }
    }

    if max_score != 0_f64 {
        return Some(str_with_max_score);
    }

    None
}

fn make_guess(input: &str) -> String {
    let words: &Vec<String> = WORDS_CELL.get_or_init(|| {
        get_word_list()
    });

    let prefix: String = "Did you mean: ".to_string();
    match find_similar_word_in_list(input, words) {
        Some(w) => format!("\x1b[33;1m{}\"{}\"?\x1b[0m", prefix, w),
        None => format!("\x1b[33;1mSorry... I could't guess~\x1b[0m")
    }
}

/// Using `print!` macro to print given content, but without expend the macro everywhere.
#[cold]
#[inline(never)]
fn basic_print<T: Display>(s: T) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

fn main() {
    let typed_string: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let typed_string_clone = typed_string.clone();
    let quit_pressed: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let quit_pressed_clone = quit_pressed.clone();
    let cursor_pos = Arc::new(Mutex::new((0_u16, 0_u16)));
    let cursor_pos_clone = cursor_pos.clone();

    // Switch the terminal to raw mode
    let mut stdout = io::stdout()
        .into_raw_mode()
        .expect("Fail to change terminal mode.");

    basic_print("Loading Resources...");
    let _words = get_word_list();
    print!("\r{}", termion::clear::CurrentLine);
    io::stdout().flush().unwrap();

    basic_print("\nType anything, I will guess what's in your mind~\n\r\
                Press [ESC] to quit.\n\n\r");

    // Thread to read input and prints it out on console
    let input_handler = thread::spawn(move || {
        let mut buf = [0_u8; 1];
        while stdin().read(&mut buf).expect("Fail to read input.") == 1 {
            let key = buf[0];
            match buf[0] {
                27 => {
                    quit_pressed.store(true, Ordering::Relaxed);
                    basic_print("\n\rLeaving...\n\r");
                    break;
                }
                127 => {
                    print!("{}{}", cursor::Left(1), termion::clear::AfterCursor);
                    pop_char(&typed_string);
                    io::stdout().flush().unwrap();

                    // save cursor pos
                    let mut pos = cursor_pos.lock().unwrap();
                    *pos = stdout
                        .cursor_pos()
                        .expect("Fail to get current cursor position.");
                }
                x if (32..=126).contains(&key) => {
                    let ch: char = x as char;
                    basic_print(ch);
                    add_char(&typed_string, ch);

                    // save cursor pos
                    let mut pos = cursor_pos.lock().unwrap();
                    *pos = stdout
                        .cursor_pos()
                        .expect("Fail to get current cursor position.");
                }
                _ => {}
            }
        }
    });

    // Thread to process input
    let output_handler = thread::spawn(move || {
        const SLEEP_TIME: Duration = Duration::from_secs(2);
        let mut input_buff: String = typed_string_clone.lock().unwrap().to_string();

        while !quit_pressed_clone.load(Ordering::Relaxed) {
            let current_input = typed_string_clone.lock().unwrap().to_string();
            if input_buff != current_input {
                let o_cursor_pos = cursor_pos_clone.lock().unwrap();
                let random_funny_text = if current_input.is_empty() {
                    String::new()
                } else {
                    make_guess(&current_input)
                };

                // The logic of this print macro is:
                // Move the cursor to the begining of current line - "\r"
                // Move the cursor up by one line - "cursor::Up(1)"
                // Clear what remains on this upper line
                // print guess content
                // move the cursor back to the bottom line (which saved in input handle)
                print!(
                    "\r{}{}{}{}",
                    cursor::Up(1),
                    termion::clear::CurrentLine,
                    random_funny_text,
                    cursor::Goto(o_cursor_pos.0, o_cursor_pos.1)
                );
                // move cursor back

                input_buff = current_input;
                io::stdout().flush().unwrap();
            }

            thread::sleep(SLEEP_TIME);
        }
    });

    input_handler
        .join()
        .expect("Error happened in input thread.");
    output_handler
        .join()
        .expect("Something went wrong in stdout reader thread.");
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_strsim() {
        let score = normalized_levenshtein("coward", "cow");
        assert!(score > 0.4);
    }

    #[test]
    fn test_finding_similar_word() {
        let words = get_word_list();
        let result = find_similar_word_in_list("chad", &words);
        assert_ne!(result, None);
    }
}