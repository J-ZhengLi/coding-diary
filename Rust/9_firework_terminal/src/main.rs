//! In order to display a nice firework scene, I need to do the following:
//! 1. Switch the terminal to raw mode using termion.
//! 2. Clear the whole screen.
//! 3. Print some static background at the buttom.
//! 4. Start a loop with a small sleep timer.
//! 5. In the loop, start to rendering fireworks with different color.
//!    Firework will be rendered from bottom up, while also attempt to restore original
//!    content of the current cursor position (if any).

mod background;
mod firework;

use background::{Building, BuildingError, BuildingLightMode};
use rand::{thread_rng, Rng};
use std::fmt::Display;
use std::io::{stdin, stdout, Stdout, Write};
use std::{thread, time::Duration};
use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

#[cold]
#[inline(never)]
fn write<T: Display>(stdout: &mut RawTerminal<Stdout>, msg: T) {
    write!(stdout, "{}", msg).expect("Fail to write to the terminal.");
    stdout.flush().unwrap();
}

/// Drawing multiple buildings as background in the current terminal window.
/// Requires a reference of raw mode terminal.
fn draw_buildings(
    stdout: &mut RawTerminal<Stdout>,
    default_width: u16,
    default_height: u16,
    max_gap: Option<u16>,
) -> Result<(), BuildingError> {
    // Get terminal size, we don't wanna construct buildings that exceeding that size
    let (max_width, max_height) =
        termion::terminal_size().unwrap_or((default_width, default_height));
    // Also, make sure the drawing start from the left-bottom corner
    write(stdout, termion::cursor::Goto(0, max_height));
    let mut used_width = 0_u16;
    let mut rng = thread_rng();

    while used_width < max_width {
        // Init a new building with a height that does not exceed terminal height / 4
        // and a width that does not exceed terminal width / 10
        let random_width: u16 = rng.gen_range(2..=max_width / 20);
        // The width of building does not counting the 2 symbols that representing walls
        // so, by adding it back will make sure there are no overlapping.
        if used_width + (random_width + 2) > max_width {
            break;
        }
        used_width += random_width + 2;
        let random_height: u16 = rng.gen_range(2..=max_height / 3);
        let has_large_wind: bool = rng.gen_bool(0.25);

        let building = Building::new(random_height, random_width)
            .use_large_windows(has_large_wind)
            .light_mode(BuildingLightMode::Random);

        // Draw building
        building.construct(stdout)?;

        if let Some(gap_limit) = max_gap {
            let gap: u16 = rng.gen_range(0..=gap_limit);
            write(stdout, " ".repeat(gap as usize));
            used_width += gap;
        }
    }
    Ok(())
}

fn main() {
    // Step 1, mode switching
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Step 2, clear screen
    write(&mut stdout, clear::All);

    // Step 3, draw static buildings as background
    draw_buildings(&mut stdout, 120, 20, Some(2)).expect("Fail to draw background buildings.");

    // Step 4, start loop, setup keyboard event handler that can break the loop,
    // and add a small sleep timer to limit "framerate", roughly 30 fps
    const FIXED_DELTA_TIME: Duration = Duration::from_millis(33);
    let mut key_itor = stdin().keys();
    'scene: loop {
        // detect key press
        if let Some(Ok(key)) = key_itor.next() {
            match key {
                Key::Esc | Key::Ctrl('c') => {
                    break 'scene;
                }
                _ => {}
            }
        }
        

        // Step 5, render firework in the loop

        thread::sleep(FIXED_DELTA_TIME);
    }
}
