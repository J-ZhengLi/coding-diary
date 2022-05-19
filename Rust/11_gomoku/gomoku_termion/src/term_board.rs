use gomoku_util::board::{Alignment, Board, GameStatus, Player};
use gomoku_util::Point;
use std::collections::hash_map::Entry;
use std::{
    cmp::{max, min},
    fmt::{Display, Write},
    io::Stdout,
};
use termion::{clear, terminal_size};

use crate::common::{debug, rtwrite, write_at_with_center_alignment};
use crate::{DetectCursorPos, Goto, RawTerminal};

pub struct TermBoard {
    pub board: Board,
    pub start_pos: Point,
    pub center_pos: Point,
    boundary: (u16, u16, u16, u16),
    game_running: bool,
}

/// Unicode characters representing parts of a Gomoku board
pub enum BoardComponent {
    BlackPiece,
    BlackPieceHL,
    WhitePiece,
    WhitePieceHL,
    BoarderTop,
    BoarderBottom,
    BoarderLeft,
    BoarderRight,
    BoarderTopLeft,
    BoarderTopRight,
    BoarderBottomLeft,
    BoarderBottomRight,
    Intersection,
    IntersectionHL,
}

impl Display for BoardComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type BC = BoardComponent;
        let c = match self {
            BC::BlackPiece => '\u{25CF}',
            BC::BlackPieceHL => '\u{25CF}',
            BC::WhitePiece => '\u{25CB}',
            BC::WhitePieceHL => '\u{25CB}',
            BC::BoarderTop => '\u{2564}',
            BC::BoarderBottom => '\u{2567}',
            BC::BoarderLeft => '\u{255F}',
            BC::BoarderRight => '\u{2562}',
            BC::BoarderTopLeft => '\u{2554}',
            BC::BoarderTopRight => '\u{2557}',
            BC::BoarderBottomLeft => '\u{255A}',
            BC::BoarderBottomRight => '\u{255D}',
            BC::Intersection => '\u{253C}',
            BC::IntersectionHL => '\u{254B}',
        };
        f.write_char(c)
    }
}

fn get_start_pos(board: &mut Board) -> (u16, u16) {
    let max_allowed_size = terminal_size().unwrap_or_default();

    // adjust board size base on window size
    let w = min(board.width, max_allowed_size.0.saturating_sub(2));
    let h = min(board.height, max_allowed_size.1.saturating_sub(4));
    board.resize(w, h);

    let center = Point::new(max_allowed_size.0 / 2, max_allowed_size.1 / 2);

    match board.alignment {
        Alignment::Left => (1, max(center.y.saturating_sub(board.height / 2), 1)),
        Alignment::Center => (
            max(center.x.saturating_sub(board.width / 2), 1),
            max(center.y.saturating_sub(board.height / 2), 1),
        ),
        Alignment::Right => (
            max(
                max_allowed_size
                    .0
                    .saturating_sub(board.width.saturating_add(2)),
                1,
            ),
            max(center.y.saturating_sub(board.height / 2), 1),
        ),
    }
}

impl TermBoard {
    pub fn new(width: u16, height: u16) -> Self {
        let mut board = Board::new(width, height);
        let startpos = get_start_pos(&mut board);
        let boundary = (
            startpos.0,
            startpos.0 + 1 + board.width,
            startpos.1,
            startpos.1 + 1 + board.height,
        );
        let center = Point {
            x: boundary.0.saturating_add(boundary.1) / 2,
            y: boundary.2.saturating_add(boundary.3) / 2,
        };

        Self {
            board,
            start_pos: startpos.into(),
            center_pos: center,
            boundary,
            game_running: true,
        }
    }

    /// Display a new board
    pub fn show(&self, out: &mut RawTerminal<Stdout>) {
        let board = &self.board;

        // Draw the board from top to buttom, left to right
        for h in 0..board.height + 2 {
            rtwrite(
                Goto(self.start_pos.x, self.start_pos.y.saturating_add(h)),
                out,
            );
            for w in 0..board.width + 2 {
                let char_to_draw = match (w, h) {
                    (0, 0) => BoardComponent::BoarderTopLeft,
                    (0, h) if h == board.height + 1 => BoardComponent::BoarderBottomLeft,
                    (w, 0) if w == board.width + 1 => BoardComponent::BoarderTopRight,
                    (w, h) if w == board.width + 1 && h == board.height + 1 => {
                        BoardComponent::BoarderBottomRight
                    }
                    (_, 0) => BoardComponent::BoarderTop,
                    (_, h) if h == board.height + 1 => BoardComponent::BoarderBottom,
                    (0, _) => BoardComponent::BoarderLeft,
                    (w, _) if w == board.width + 1 => BoardComponent::BoarderRight,
                    _ => BoardComponent::Intersection,
                };
                rtwrite(char_to_draw, out);
            }
        }
    }

    pub fn start(&mut self, out: &mut RawTerminal<Stdout>) {
        self.start_with_player(out, Some(&Player::Black));
    }

    pub fn start_with_player(
        &mut self,
        out: &mut RawTerminal<Stdout>,
        start_player: Option<&Player>,
    ) {
        self.board.init(start_player);
        rtwrite(clear::All, out);
        self.show(out);
        self.move_to_center(out);
        self.game_running = true;
    }

    fn move_by(&self, distance: (i32, i32), out: &mut RawTerminal<Stdout>) {
        if let Ok(cur_pos) = out.cursor_pos() {
            let dest = Point::from(cur_pos) + distance;
            self.move_to(dest, out)
        }
    }

    /// Check if given x, y cordinate is a valid point of current board
    fn is_valid_pos(&self, pos: Point) -> bool {
        let (min_x, max_x, min_y, max_y) = self.boundary;
        pos.x > min_x && pos.x < max_x && pos.y > min_y && pos.y < max_y
    }

    pub fn move_up(&self, out: &mut RawTerminal<Stdout>) {
        self.move_by((0, -1), out)
    }

    pub fn move_down(&self, out: &mut RawTerminal<Stdout>) {
        self.move_by((0, 1), out)
    }

    pub fn move_left(&self, out: &mut RawTerminal<Stdout>) {
        self.move_by((-1, 0), out)
    }

    pub fn move_right(&self, out: &mut RawTerminal<Stdout>) {
        self.move_by((1, 0), out)
    }

    pub fn move_to(&self, dest: Point, out: &mut RawTerminal<Stdout>) {
        if self.is_valid_pos(dest) {
            rtwrite(Goto(dest.x, dest.y), out);
        }
    }

    pub fn move_to_center(&self, out: &mut RawTerminal<Stdout>) {
        self.move_to(self.center_pos, out)
    }

    pub fn place_pawn(&mut self, out: &mut RawTerminal<Stdout>) {
        if !self.game_running {
            return;
        }

        if let Ok(cur_pos) = out.cursor_pos() {
            if let Entry::Vacant(_) = self.board.player_pos.entry(cur_pos) {
                match self.board.cur_player {
                    Player::Black => {
                        rtwrite(BoardComponent::BlackPiece, out);
                        self.board.player_pos.insert(cur_pos, Player::Black);
                    }
                    Player::White => {
                        rtwrite(BoardComponent::WhitePiece, out);
                        self.board.player_pos.insert(cur_pos, Player::White);
                    }
                }
                self.move_to(cur_pos.into(), out);
                self.board.empty_count -= 1;
                debug(
                    format!(
                        "game status: {:?}: who's: {}, pos: {:?}, pos remaining: {}",
                        self.board
                            .get_game_status(cur_pos.into(), self.board.cur_player),
                        self.board.cur_player,
                        cur_pos,
                        self.board.empty_count,
                    ),
                    out,
                );
                if let GameStatus::Over(winner) = self
                    .board
                    .get_game_status(cur_pos.into(), self.board.cur_player)
                {
                    let msg = if let Some(winner) = winner {
                        format!("Game Over\n[{} Wins!]\n\nPress <R> to restart", winner)
                    } else {
                        format!("Game Over\n[Draw]\n\nPress <R> to restart")
                    };
                    write_at_with_center_alignment(msg, out, self.center_pos.into());
                    self.game_running = false;
                }
                self.board.switch_player();
            }
        }
    }
}
