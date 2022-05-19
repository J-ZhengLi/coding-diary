use std::{collections::HashMap, fmt::Display};

use crate::Point;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

#[derive(Debug)]
pub enum GameStatus {
    Running,
    Over(Option<Player>),
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Player::Black => "player: Black",
            Player::White => "player: White",
        };

        f.write_str(name)
    }
}

#[derive(Debug)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct Board {
    pub width: u16,
    pub height: u16,
    pub cur_player: Player,
    pub alignment: Alignment,
    pub player_pos: HashMap<(u16, u16), Player>,
    pub empty_count: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            width: 15,
            height: 15,
            cur_player: Player::Black,
            alignment: Alignment::Center,
            player_pos: HashMap::new(),
            empty_count: 15 * 15,
        }
    }
}

impl Board {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn switch_player(&mut self) {
        match self.cur_player {
            Player::Black => self.cur_player = Player::White,
            Player::White => self.cur_player = Player::Black,
        }
    }

    /// Initializing the game board
    ///
    /// Take ownership of starting player if provided.
    pub fn init(&mut self, start_player: Option<&Player>) {
        self.player_pos.clear();
        self.empty_count = (self.width * self.height) as usize;
        if let Some(p) = start_player {
            self.cur_player = *p;
        }
    }

    /// Adjust the current board's dimension
    ///
    /// Note this will also clear everything in it, including player's pawns
    /// that are already placed on board.
    pub fn resize(&mut self, new_width: u16, new_height: u16) {
        self.width = new_width;
        self.height = new_height;
        self.init(None);
    }

    fn check_common(&self, pos: Point, dir_unit: (i32, i32), player: Player) -> u16 {
        let mut count: u16 = 0;
        let mut i = 1;

        // This loop breaks when given position is empty or occupied by different player
        loop {
            let neighbor_pos = pos + (i * dir_unit.0, i * dir_unit.1);
            i += 1;

            match self.player_pos.get(&(neighbor_pos.x, neighbor_pos.y)) {
                Some(p) if *p == player => {
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
    fn horizontal_check(&self, pos: Point, player: Player) -> u16 {
        1 + self.check_common(pos, (-1, 0), player) + self.check_common(pos, (1, 0), player)
    }

    /// Check for stones line up in "|" direction
    fn vertical_check(&self, pos: Point, player: Player) -> u16 {
        1 + self.check_common(pos, (0, -1), player) + self.check_common(pos, (0, 1), player)
    }

    /// Check for stones line up in "/" direction
    fn forward_diag_check(&self, pos: Point, player: Player) -> u16 {
        1 + self.check_common(pos, (1, -1), player) + self.check_common(pos, (-1, 1), player)
    }

    /// Check for stones line up in "\" direction
    fn backward_diag_check(&self, pos: Point, player: Player) -> u16 {
        1 + self.check_common(pos, (-1, -1), player) + self.check_common(pos, (1, 1), player)
    }

    pub fn check(&self, pos: Point, player: Player, target_amount: u16) -> bool {
        self.horizontal_check(pos, player) >= target_amount
            || self.vertical_check(pos, player) >= target_amount
            || self.forward_diag_check(pos, player) >= target_amount
            || self.backward_diag_check(pos, player) >= target_amount
    }

    pub fn get_game_status(&self, pos: Point, player: Player) -> GameStatus {
        if self.check(pos, player, 5) {
            GameStatus::Over(Some(player))
        } else if self.empty_count == 0 {
            GameStatus::Over(None)
        } else {
            GameStatus::Running
        }
    }
}
