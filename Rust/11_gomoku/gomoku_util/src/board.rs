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
    pub highlight_pos: (u16, u16),
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
            highlight_pos: (15 / 2, 15 / 2),
            alignment: Alignment::Center,
            player_pos: HashMap::new(),
            empty_count: 15 * 15
        }
    }
}

impl Board {
    pub fn new(width: u16, height: u16) -> Self {
        let highlight_pos = (width / 2 + 1, height / 2 + 1);
        Self {
            width,
            height,
            highlight_pos,
            ..Default::default()
        }
    }

    pub fn switch_player(&mut self) {
        match self.cur_player {
            Player::Black => self.cur_player = Player::White,
            Player::White => self.cur_player = Player::Black,
        }
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
            return GameStatus::Over(Some(player));
        } else {
            if self.empty_count == 0 {
                return GameStatus::Over(None);
            }
            return GameStatus::Running;
        }
    }
}
