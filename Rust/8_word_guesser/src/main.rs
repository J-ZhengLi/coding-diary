//! # Word Guesser
//!
//! In order to guess word as user type,
//! the program needs to have 3 threads.
//! One for the main thread, which shows input and output,
//! one for reading user input,
//! and one for processing the guesses.

use std::io::{self, stdin, Read, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::{thread, time::Duration};
use termion::raw::IntoRawMode;

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

fn main() {
    let typed_string: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let typed_string_clone = typed_string.clone();
    let quit_pressed: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let quit_pressed_clone = quit_pressed.clone();
    println!("\nType anything, I will guess what's in your mind~\nPress [ESC] to quit.");

    // Thread to read input and prints it out on console
    let input_handler = thread::spawn(move || {
        let mut buf = [0_u8; 1];
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        while stdin().read(&mut buf).expect("Fail to read input.") == 1 {
            let key = buf[0];
            match buf[0] {
                27 => {
                    quit_pressed.store(true, Ordering::Relaxed);
                    write!(stdout, "\n\rLeaving...\n\r").unwrap();
                    break;
                }
                127 => {
                    write!(
                        stdout,
                        "{}{}",
                        termion::cursor::Left(1),
                        termion::clear::AfterCursor
                    )
                    .unwrap();
                    pop_char(&typed_string);
                    io::stdout().flush().unwrap();
                }
                x if key <= 126 && key >= 32 => {
                    let ch: char = x as char;
                    write!(stdout, "{}", ch).unwrap();
                    add_char(&typed_string, ch);
                    io::stdout().flush().unwrap();
                }
                _ => {}
            }
        }
    });

    // Thread to process input
    let output_handler = thread::spawn(move || {
        let sleep_time = Duration::from_secs(1);
        let mut input_buff: String = typed_string_clone.lock().unwrap().to_string();

        while !quit_pressed_clone.load(Ordering::Relaxed) {
            let current_input = typed_string_clone.lock().unwrap().to_string();
            if input_buff != current_input {
                print!("{}", current_input);
                input_buff = current_input;
                io::stdout().flush().unwrap();
            }

            thread::sleep(sleep_time);
        }
    });

    input_handler
        .join()
        .expect("Error happened in input thread.");
    output_handler
        .join()
        .expect("Something went wrong in stdout reader thread.");
}
