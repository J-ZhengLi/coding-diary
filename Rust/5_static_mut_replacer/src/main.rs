use std::sync::Mutex;
use std::sync::atomic::{AtomicU16, Ordering::SeqCst};

use once_cell::sync::Lazy;
use lazy_static::lazy_static;

/// Global character level, requires mutability for leveling up.
static CHAR_LEVEL: AtomicU16 = AtomicU16::new(0);

static CHAR_TITLE: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new("Newbie".to_string())
});

fn main() {
    println!("Hello, world!");
}
