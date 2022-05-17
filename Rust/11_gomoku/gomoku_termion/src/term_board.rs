use gomoku_util::board::{Alignment, Board, Player};
use std::{
    cmp::max,
    fmt::{Display, Write},
    io::Stdout,
};
use termion::{raw::RawTerminal, terminal_size};

use crate::{debug, rtwrite};
use crate::{DetectCursorPos, Goto};

pub struct TermBoard {
    pub board: Board,
    pub start_pos: (u16, u16),
    boundary: (u16, u16, u16, u16),
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

fn _get_start_pos(board: &Board) -> (u16, u16) {
    let max_allowed_size = if let Ok(size) = terminal_size() {
        size
    } else {
        return (0, 0);
    };

    let center_point = (max_allowed_size.0 / 2, max_allowed_size.1 / 2);

    match board.alignment {
        Alignment::Left => (1, max(center_point.1 - board.height / 2 - 1, 1)),
        Alignment::Center => (
            max(center_point.0 - board.width / 2 - 1, 1),
            max(center_point.1 - board.height / 2 - 1, 1),
        ),
        Alignment::Right => (
            max(max_allowed_size.0 - board.width + 2, 1),
            max(center_point.1 - board.height / 2 - 1, 1),
        ),
    }
}

impl TermBoard {
    pub fn new(width: u16, height: u16) -> Self {
        let board = Board::new(width, height);
        let startpos = _get_start_pos(&board);
        let boundary = (
            startpos.0,
            startpos.0 + 1 + board.width,
            startpos.1,
            startpos.1 + 1 + board.height,
        );

        Self {
            board,
            start_pos: startpos,
            boundary,
        }
    }

    pub fn show(&self, out: &mut RawTerminal<Stdout>) {
        let (startpos_x, startpos_y) = self.start_pos;
        let board = &self.board;

        // Draw the board from top to buttom, left to right
        for h in 0..board.height + 2 {
            rtwrite(Goto(startpos_x, startpos_y + h), out);
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

    fn _move_by_one(&self, delta_x: i32, delta_y: i32, out: &mut RawTerminal<Stdout>) {
        if let Ok(cur_pos) = out.cursor_pos() {
            let dest_x = cur_pos.0 as i32 + delta_x;
            let dest_y = cur_pos.1 as i32 + delta_y;
            if dest_x > 0 && dest_y > 0 {
                self.move_to(dest_x as u16, dest_y as u16, out)
            }
        }
    }

    /// Check if given x, y cordinate is a valid point of current board
    fn _is_valid_pos(&self, x: u16, y: u16) -> bool {
        let (min_x, max_x, min_y, max_y) = self.boundary;
        x > min_x && x < max_x && y > min_y && y < max_y
    }

    pub fn move_up(&self, out: &mut RawTerminal<Stdout>) {
        self._move_by_one(0, -1, out)
    }

    pub fn move_down(&self, out: &mut RawTerminal<Stdout>) {
        self._move_by_one(0, 1, out)
    }

    pub fn move_left(&self, out: &mut RawTerminal<Stdout>) {
        self._move_by_one(-1, 0, out)
    }

    pub fn move_right(&self, out: &mut RawTerminal<Stdout>) {
        self._move_by_one(1, 0, out)
    }

    pub fn move_to(&self, x: u16, y: u16, out: &mut RawTerminal<Stdout>) {
        if self._is_valid_pos(x, y) {
            rtwrite(Goto(x, y), out);
        }
    }

    pub fn move_to_center(&self, out: &mut RawTerminal<Stdout>) {
        let (center_x, center_y) = (
            self.start_pos.0 + self.board.highlight_pos.0,
            self.start_pos.1 + self.board.highlight_pos.1,
        );
        self.move_to(center_x, center_y, out)
    }

    pub fn place_pawn(&mut self, out: &mut RawTerminal<Stdout>) {
        if let Ok(cur_pos) = out.cursor_pos() {
            if !self.board.player_pos.contains_key(&cur_pos) {
                debug(format!("{}: {}", &self.board.check(cur_pos, &self.board.cur_player), self.board.cur_player), out);
                match self.board.cur_player {
                    Player::Black => {
                        rtwrite(BoardComponent::BlackPiece, out);
                        self.board.player_pos.insert(cur_pos, Player::Black);
                        self.board.switch_player();
                    }
                    Player::White => {
                        rtwrite(BoardComponent::WhitePiece, out);
                        self.board.player_pos.insert(cur_pos, Player::White);
                        self.board.switch_player();
                    }
                }
            }
        }
    }
}
