use gomoku_termion::start_game;
use gomoku_termion::terminal_setting::{CursorMode, TermSettings};

fn main() {
    let t_settings = TermSettings::new().cursor_mode(CursorMode::SteadyBlock);
    start_game(None, Some(t_settings)).expect("Unable to launch new game due to unknown error.");
}
