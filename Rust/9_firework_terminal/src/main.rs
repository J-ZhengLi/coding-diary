//! In order to display a nice firework scene, I need to do the following:
//! 1. Switch the terminal to raw mode using termion.
//! 2. Clear the whole screen.
//! 3. Print some static background at the buttom.
//! 4. Start a loop with a small sleep timer.
//! 5. In the loop, start to rendering fireworks with different color.
//!    Firework will be rendered from bottom up, while also attempt to restore original
//!    content of the current cursor position (if any).

mod background;

use background::{Building, BuildingLightMode};
use std::fmt::Display;
use std::io::{stdout, Stdout, Write};
use termion::clear;
use termion::raw::{IntoRawMode, RawTerminal};

#[cold]
#[inline(never)]
fn write<T: Display>(stdout: &mut RawTerminal<Stdout>, msg: T) {
    write!(stdout, "{}", msg).expect("Fail to write to the terminal.");
    stdout.flush().unwrap();
}

fn main() {
    println!("Hello, world!");

    // Step 1, mode switching
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Step 2, clear screen
    write(&mut stdout, clear::All);

    // Test building
    let test_building = Building::new(10, 4)
        .use_large_windows(false)
        .light_mode(BuildingLightMode::Random);
    let _ = test_building.construct(&mut stdout);
}
