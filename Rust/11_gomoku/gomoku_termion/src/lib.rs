mod common;
pub mod game_settings;
pub mod term_board;
pub mod terminal_setting;

use common::{debug, rtwrite};
use game_settings::GameSettings;
use std::io::{stdin, stdout, Error, Stdout};
use std::thread;
use std::time::Duration;
use term_board::TermBoard;
use terminal_setting::TermSettings;
use termion::clear;
use termion::cursor::{DetectCursorPos, Goto};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

/// Switch current terminal mode into raw, and clear the sceen
fn to_raw_mode(settings: Option<TermSettings>) -> Result<RawTerminal<Stdout>, Error> {
    let mut s_out = stdout().into_raw_mode()?;
    if let Some(ts) = settings {
        rtwrite(ts, &mut s_out);
    }
    return Ok(s_out);
}

pub fn start_game(
    game_settings: Option<GameSettings>,
    terminal_setting: Option<TermSettings>,
) -> Result<(), Error> {
    let g_settings = game_settings.unwrap_or_default();
    let delta_time: Duration = Duration::from_secs_f32(f32::from(1_u16 / g_settings.fps));
    let mut out = to_raw_mode(terminal_setting).expect("Unable to switch to raw mode.");
    let mut term_board = TermBoard::new(g_settings.board_size.0, g_settings.board_size.1);

    term_board.show(&mut out);
    term_board.move_to_center(&mut out);

    'game: loop {
        for key in stdin().keys() {
            let k = key?;
            match k {
                Key::Esc | Key::Ctrl('c') => break 'game,
                Key::Up => term_board.move_up(&mut out),
                Key::Down => term_board.move_down(&mut out),
                Key::Left => term_board.move_left(&mut out),
                Key::Right => term_board.move_right(&mut out),
                Key::Char(' ') => term_board.place_pawn(&mut out),
                _ => {}
            }
        }

        thread::sleep(delta_time);
    }

    rtwrite(
        format!("{}{}{}", TermSettings::default(), Goto(1, 1), clear::All),
        &mut out,
    );
    Ok(())
}
