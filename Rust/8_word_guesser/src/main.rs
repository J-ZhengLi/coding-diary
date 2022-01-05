//! # Word Guesser
//! 
//! In order to guess word as user type,
//! the program needs to have 3 threads.
//! One for the main thread, which shows input and output,
//! one for reading user input,
//! and one for processing the guesses.

use termion::raw::IntoRawMode;
use std::io::{self, Write, Read, stdin};
use std::thread;

fn main() {
    println!("\nType anything, I will guess what's in your mind~\n");

    // Thread to read input
    let input_handler = thread::spawn(move || {
        let mut buf = [0_u8; 1];
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        while stdin().read(&mut buf).expect("Fail to read input.") == 1 {
            let key = buf[0];
            match buf[0] {
                27 => {
                    write!(stdout, "\n\r").unwrap();
                    break;
                }
                127 => {
                    write!(stdout, "{}{}", termion::cursor::Left(1), termion::clear::AfterCursor).unwrap();
                    io::stdout().flush().unwrap();
                },
                x if key <= 126 && key >= 32 => {
                    write!(stdout, "{}", x as char).unwrap();
                    io::stdout().flush().unwrap();
                }
                _ => {}
            }
        }
    });

    input_handler.join().unwrap();
}