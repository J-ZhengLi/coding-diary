use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Player {
    Black,
    White,
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

    fn _check_common(
        &self,
        pos: (u16, u16),
        bound: (u16, u16),
        side_to_check: &str,
        player: &Player,
    ) -> u16 {
        let mut count: u16 = 0;
        let mut i = 1;
        for _ in bound.0..bound.1 {
            let neighbor_pos = match side_to_check {
                "l" => (pos.0 - i, pos.1),
                "r" => (pos.0 + i, pos.1),
                "u" => (pos.0, pos.1 - 1),
                "d" => (pos.0, pos.1 + 1),
                "ul" => (pos.0 - i, pos.1 - 1),
                "ur" => (pos.0 + i, pos.1 - 1),
                "dl" => (pos.0 - i, pos.1 + 1),
                "dr" => (pos.0 + i, pos.1 + 1),
                _ => panic!("build failed, invalid side input"),
            };

            i += 1;

            match self.player_pos.get(&neighbor_pos) {
                Some(p) if p == player => {
                    count += 1;
                }
                _ => {
                    break;
                }
            }
        }
        count
    }

    /// Check for neighbor stones on the left and on the right, and return how many stones
    /// that have same color in a line
    fn _horizontal_check(&self, pos: (u16, u16), player: &Player) -> u16 {
        1 + self._check_common(pos, (0, pos.0), "l", player)
            + self._check_common(pos, (pos.0 + 1, self.width), "r", player)
    }

    fn _vertical_check(&self, pos: (u16, u16), player: &Player) -> u16 {
        1 + self._check_common(pos, (0, pos.1), "u", player)
            + self._check_common(pos, (pos.1 + 1, self.height), "d", player)
    }

    fn _forward_diag_check(&self, pos: (u16, u16), player: &Player) -> bool {
        false
    }

    fn _backward_diag_check(&self, pos: (u16, u16), player: &Player) -> bool {
        false
    }

    pub fn check(&self, pos: (u16, u16), player: &Player) -> u16 {
        let h_count = self._horizontal_check(pos, player);
        let v_count = self._vertical_check(pos, player);

        h_count
    }
}

pub trait BoardControls {
    fn move_up(&self);
    fn move_down(&self);
    fn move_left(&self);
    fn move_right(&self);
    /// Move to any point in the board, (0, 0) representing the top left of the board
    fn move_to(&self, to: (u16, u16));
    fn place_pawn(&self, player: Player, position: Option<(u16, u16)>);
}
