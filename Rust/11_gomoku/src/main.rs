use gomoku_term::start_game;

fn main() {
    start_game(30).expect("Unable to launch new game due to unknown error.");
}
