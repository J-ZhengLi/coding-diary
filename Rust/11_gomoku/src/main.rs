use gomoku_termion::start_game;
use gomoku_termion::terminal_setting::{CursorMode, TermSettings, TextStyle};

fn main() {
    let t_settings = TermSettings::new(true)
        .text_style(TextStyle::Bold)
        .cursor_mode(CursorMode::SteadyBlock);
    start_game(None, Some(t_settings)).expect("Unable to launch new game due to unknown error.");
}
