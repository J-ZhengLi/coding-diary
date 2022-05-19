mod common;
pub mod settings;
pub mod term_board;

use common::write;
use crossterm::cursor::{
    CursorShape, DisableBlinking, MoveTo, SetCursorShape,
};
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use settings::{GameSettings, TermSettings};
use std::io::{stdout, Error};
use std::thread;
use std::time::Duration;
use term_board::TermBoard;

pub fn start_game(game_settings: Option<GameSettings>) -> Result<(), Error> {
    let g_settings = game_settings.unwrap_or_default();
    let delta_time: Duration = Duration::from_secs_f32(f32::from(1_u16 / g_settings.fps));
    enable_raw_mode()?;
    execute!(
        stdout(),
        DisableBlinking,
        SetCursorShape(CursorShape::Block)
    )?;
    let mut term_board = TermBoard::new(g_settings.board_size.0, g_settings.board_size.1);

    term_board.start();

    'game: loop {
        if let Event::Key(ke) = read()? {
            match ke.code {
                KeyCode::Up => term_board.move_up(),
                KeyCode::Down => term_board.move_down(),
                KeyCode::Left => term_board.move_left(),
                KeyCode::Right => term_board.move_right(),
                KeyCode::Char('r') => term_board.start(),
                KeyCode::Char(' ') => term_board.place_pawn(),
                KeyCode::Char('c') if ke.modifiers == KeyModifiers::CONTROL => break 'game,
                KeyCode::Esc => break 'game,
                _ => {}
            }
        }

        thread::sleep(delta_time);
    }

    // Game stopped, reset terminal
    write(format!(
        "{}{}{}",
        TermSettings::default(),
        MoveTo(0, 0),
        Clear(ClearType::All)
    ));

    disable_raw_mode()?;
    Ok(())
}
