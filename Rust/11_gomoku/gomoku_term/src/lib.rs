pub mod common;
pub mod settings;
pub mod term_board;

use anyhow::Result;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use std::thread;
use std::time::Duration;
use term_board::TermBoard;

pub fn start_game(fps: u16) -> Result<()> {
    let delta_time: Duration = Duration::from_secs_f32(f32::from(1_u16 / fps));
    let mut term_board = TermBoard::init_with_default();
    term_board.start()?;

    'game: loop {
        if let Event::Key(ke) = read()? {
            match ke.code {
                KeyCode::Up => term_board.move_cursor((0, -1))?,
                KeyCode::Down => term_board.move_cursor((0, 1))?,
                KeyCode::Left => term_board.move_cursor((-1, 0))?,
                KeyCode::Right => term_board.move_cursor((1, 0))?,
                KeyCode::Char('r') => term_board.start()?,
                KeyCode::Char(' ') => term_board.place_pawn()?,
                KeyCode::Char('c') if ke.modifiers == KeyModifiers::CONTROL => break 'game,
                KeyCode::Esc => break 'game,
                _ => (),
            }
        }

        thread::sleep(delta_time);
    }
    Ok(())
}
