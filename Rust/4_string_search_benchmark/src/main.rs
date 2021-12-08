use std::time::Instant;

fn main() {
    // load text content
    // Hamlet - full text, contains 194354 characters
    let full_text = include_str!("../res/hamlet.txt");

    // start timer fro memchr
    let sw = Instant::now();

    println!("Application started at: {}", sw.elapsed().as_millis());
    println!("{}", full_text.len());
}
