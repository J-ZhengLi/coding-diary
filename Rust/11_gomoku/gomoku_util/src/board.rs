use std::{collections::HashMap, fmt::Display};

use crate::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player<'b> {
    Black(&'b str),
    White(&'b str),
}

#[derive(Debug)]
pub enum GameStatus<'b> {
    Running,
    Over(Option<Player<'b>>),
}

impl<'b> Display for Player<'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Player::Black(name) => format!("[Black] {name}"),
            Player::White(name) => format!("[White] {name}"),
        };

        f.write_str(&name)
    }
}

#[derive(Debug)]
pub struct Board<'b> {
    pub width: u16,
    pub height: u16,
    pub players: [Player<'b>; 2],
    cur_player_idx: u8,
    /// Player in this hashmap is represented by a single index,
    /// an intersection with no player will be represented as -1.
    pub board_pos: HashMap<Point, i16>,
    pub empty_count: usize,
}

impl<'b> Default for Board<'b> {
    fn default() -> Self {
        Self {
            width: 15,
            height: 15,
            players: [Player::Black("player 0"), Player::White("player 1")],
            cur_player_idx: 0,
            board_pos: init_board_pos(15, 15),
            empty_count: 15 * 15,
        }
    }
}

impl<'b> Board<'b> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            empty_count: (width * height) as usize,
            board_pos: init_board_pos(width, height),
            ..Default::default()
        }
    }

    /// Basically removing all placed pawns.
    pub fn reset(&mut self) {
        self.board_pos = init_board_pos(self.width, self.height);
        self.empty_count = (self.width * self.height) as usize;
    }

    /// Adjust the current board's dimension
    ///
    /// Note this will also clear everything in it, including player's pawns
    /// that are already placed on board.
    pub fn resize(&mut self, new_width: u16, new_height: u16) {
        self.width = new_width;
        self.height = new_height;
        self.reset();
    }

    pub fn is_vacant(&self, pos: Point) -> bool {
        self.board_pos.get(&pos) == Some(&-1)
    }

    /// Place pawn on desired position.
    ///
    /// return a reference to the pawn which was inserted.
    pub fn place_pawn(&mut self, pos: Point) -> (&Player, GameStatus) {
        self.board_pos.insert(pos, self.cur_player_idx.into());
        self.empty_count -= 1;
        let player = &self.players[self.cur_player_idx as usize];
        let status = self.game_status(pos, player);
        if let GameStatus::Running = status {
            self.cur_player_idx = (self.cur_player_idx + 1) % 2;
        }
        (player, status)
    }

    fn game_status(&self, pos: Point, player: &'b Player) -> GameStatus<'b> {
        if self.check(pos, player, 5) {
            GameStatus::Over(Some(player.to_owned()))
        } else if self.empty_count == 0 {
            GameStatus::Over(None)
        } else {
            GameStatus::Running
        }
    }

    fn check_common(&self, pos: Point, dir_unit: (i32, i32), player: &'b Player) -> u16 {
        let mut count: u16 = 0;
        let mut i = 1;

        // This loop breaks when given position is empty or occupied by different player
        loop {
            let neighbor_pos = pos + (i * dir_unit.0, i * dir_unit.1);
            i += 1;

            match self.board_pos.get(&neighbor_pos) {
                Some(p) if &self.players[*p as usize] == player => {
                    count += 1;
                }
                _ => {
                    break;
                }
            }
        }
        count
    }

    /// Check for stones line up in "-" direction
    fn horizontal_check(&self, pos: Point, player: &'b Player) -> u16 {
        1 + self.check_common(pos, (-1, 0), player) + self.check_common(pos, (1, 0), player)
    }

    /// Check for stones line up in "|" direction
    fn vertical_check(&self, pos: Point, player: &'b Player) -> u16 {
        1 + self.check_common(pos, (0, -1), player) + self.check_common(pos, (0, 1), player)
    }

    /// Check for stones line up in "/" direction
    fn forward_diag_check(&self, pos: Point, player: &'b Player) -> u16 {
        1 + self.check_common(pos, (1, -1), player) + self.check_common(pos, (-1, 1), player)
    }

    /// Check for stones line up in "\" direction
    fn backward_diag_check(&self, pos: Point, player: &'b Player) -> u16 {
        1 + self.check_common(pos, (-1, -1), player) + self.check_common(pos, (1, 1), player)
    }

    pub fn check(&self, pos: Point, player: &'b Player, target_amount: u16) -> bool {
        self.horizontal_check(pos, player) >= target_amount
            || self.vertical_check(pos, player) >= target_amount
            || self.forward_diag_check(pos, player) >= target_amount
            || self.backward_diag_check(pos, player) >= target_amount
    }
}

fn init_board_pos(width: u16, height: u16) -> HashMap<Point, i16> {
    let mut res: HashMap<Point, i16> = HashMap::new();
    for h in 0..height {
        for w in 0..width {
            res.insert(Point::from((w, h)), -1);
        }
    }
    res
}
