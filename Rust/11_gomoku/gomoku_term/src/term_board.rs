use gomoku_util::board::{Alignment, Board, GameStatus, Player};
use gomoku_util::Point;
use std::collections::hash_map::Entry;
use std::thread;
use std::time::Duration;
use std::{
    cmp::min,
    fmt::{Display, Write},
};

use crate::common::{
    debug, execute, write, write_at_screen_center, write_at_with_center_alignment,
};
use crossterm::cursor::{position, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp};
use crossterm::terminal::{size, Clear, ClearType};

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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
    WhitePiece,
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
            BC::WhitePiece => '\u{25CB}',
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
    let max_allowed_size = size().unwrap_or_default();

    // adjust board size base on window size
    let w = min(board.width, max_allowed_size.0.saturating_sub(2));
    let h = min(board.height, max_allowed_size.1.saturating_sub(4));
    board.resize(w, h);

    let center = Point::new(max_allowed_size.0 / 2, max_allowed_size.1 / 2);

    match board.alignment {
        Alignment::Left => (
            1,
            center.y.saturating_sub(board.height.saturating_add(2) / 2),
        ),
        Alignment::Center => (
            center.x.saturating_sub(board.width.saturating_add(2) / 2),
            center.y.saturating_sub(board.height.saturating_add(2) / 2),
        ),
        Alignment::Right => (
            max_allowed_size
                .0
                .saturating_sub(board.width.saturating_add(2)),
            center.y.saturating_sub(board.height.saturating_add(2) / 2),
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

    pub fn new_with_default() -> Self {
        Self::new(15, 15)
    }

    /// Display a new board
    pub fn show(&self) {
        let board = &self.board;

        // Draw the board from top to buttom, left to right
        for h in 0..board.height.saturating_add(2) {
            write(MoveTo(self.start_pos.x, self.start_pos.y.saturating_add(h)));
            for w in 0..board.width.saturating_add(2) {
                let char_to_draw = match (w, h) {
                    (0, 0) => BoardComponent::BoarderTopLeft,
                    (0, h) if h == board.height.saturating_add(1) => {
                        BoardComponent::BoarderBottomLeft
                    }
                    (w, 0) if w == board.width.saturating_add(1) => BoardComponent::BoarderTopRight,
                    (w, h)
                        if w == board.width.saturating_add(1)
                            && h == board.height.saturating_add(1) =>
                    {
                        BoardComponent::BoarderBottomRight
                    }
                    (_, 0) => BoardComponent::BoarderTop,
                    (_, h) if h == board.height.saturating_add(1) => BoardComponent::BoarderBottom,
                    (0, _) => BoardComponent::BoarderLeft,
                    (w, _) if w == board.width.saturating_add(1) => BoardComponent::BoarderRight,
                    _ => BoardComponent::Intersection,
                };
                write(char_to_draw);
            }
        }
    }

    pub fn start(&mut self) {
        self.start_with_player(Some(&Player::Black));
    }

    pub fn start_with_player(&mut self, start_player: Option<&Player>) {
        self.board.init(start_player);
        execute(Clear(ClearType::All));
        write_at_screen_center("Starting new game!");
        thread::sleep(Duration::from_secs(1));
        execute(Clear(ClearType::All));
        self.show();
        self.move_to_center();
        self.game_running = true;
    }

    /// Check if given x, y cordinate is a valid point of current board
    fn is_valid_pos(&self, pos: Point) -> bool {
        let (min_x, max_x, min_y, max_y) = self.boundary;
        pos.x > min_x && pos.x < max_x && pos.y > min_y && pos.y < max_y
    }

    pub fn move_cursor(&self, distance: (i32, i32)) {
        if self.game_running {
            position().map_or((), |cur_pos| {
                let des = Point::from(cur_pos) + distance;
                if self.is_valid_pos(des) {
                    match (distance.0, distance.1) {
                        (0, y) if y.is_negative() => write(MoveUp(distance.1.abs() as u16)),
                        (0, y) if y.is_positive() => write(MoveDown(distance.1 as u16)),
                        (x, 0) if x.is_negative() => write(MoveLeft(distance.0.abs() as u16)),
                        (x, 0) if x.is_positive() => write(MoveRight(distance.0 as u16)),
                        _ => (),
                    }
                }
            })
        }
    }

    pub fn move_to_center(&mut self) {
        execute(MoveTo(self.center_pos.x, self.center_pos.y));
    }

    pub fn place_pawn(&mut self) {
        if !self.game_running {
            return;
        }

        if let Ok(cur_pos) = position() {
            if let Entry::Vacant(_) = self.board.player_pos.entry(cur_pos) {
                match self.board.cur_player {
                    Player::Black => {
                        write(BoardComponent::BlackPiece);
                        self.board.player_pos.insert(cur_pos, Player::Black);
                    }
                    Player::White => {
                        write(BoardComponent::WhitePiece);
                        self.board.player_pos.insert(cur_pos, Player::White);
                    }
                }
                write(MoveTo(cur_pos.0, cur_pos.1));
                self.board.empty_count -= 1;
                debug(format!("{}", self.board.empty_count,));
                if let GameStatus::Over(winner) = self
                    .board
                    .get_game_status(cur_pos.into(), self.board.cur_player)
                {
                    let msg = if let Some(winner) = winner {
                        format!("Game Over\n[{} Wins!]\n\nPress <R> to restart", winner)
                    } else {
                        format!("Game Over\n[Draw]\n\nPress <R> to restart")
                    };
                    write_at_with_center_alignment(msg, self.center_pos.into());
                    self.game_running = false;
                }
                self.board.switch_player();
            }
        }
    }
}
