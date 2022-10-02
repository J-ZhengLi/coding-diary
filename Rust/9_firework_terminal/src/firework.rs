//! # Make animated firework
//!
//! Different from the `background` mod,
//! this module only contains stuctures that define the properties of a `Firework`
//! without an method to output it to the window.

#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Colorful,
}

#[derive(Debug)]
pub enum FireworkCount {
    Finite(u16),
    Infinite,
}

impl From<u16> for FireworkCount {
    fn from(count: u16) -> Self {
        FireworkCount::Finite(count)
    }
}

impl PartialEq for FireworkCount {
    fn eq(&self, other: &Self) -> bool {
        match self {
            FireworkCount::Finite(a) => match other {
                FireworkCount::Finite(b) => a == b,
                FireworkCount::Infinite => false,
            },
            &FireworkCount::Infinite => match other {
                FireworkCount::Finite(_) => false,
                FireworkCount::Infinite => true,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Firework {
    size: u16,
    height: u16,
    spawn_point: (u16, u16),
    count: FireworkCount,
    color: Color,
    is_bright: bool,
    fade_delay: f32,
    trail_length: u16,
    trail_color: Color,
    is_trail_bright: bool,
}

fn calculate_spawn_point(size: u16, height: u16) -> (u16, u16) {
    ((size as f32 / 2.0).ceil() as u16, height)
}

impl Firework {
    fn new(size: u16, height: u16) -> Self {
        Firework {
            size,
            height,
            spawn_point: calculate_spawn_point(size, height),
            ..Default::default()
        }
    }

    fn new_bright(size: u16, height: u16) -> Self {
        Firework {
            size,
            height,
            spawn_point: calculate_spawn_point(size, height),
            is_bright: true,
            is_trail_bright: true,
            ..Default::default()
        }
    }

    fn new_with_color(size: u16, height: u16, c: Color) -> Self {
        Firework {
            size,
            height,
            spawn_point: calculate_spawn_point(size, height),
            color: c,
            trail_color: c,
            ..Default::default()
        }
    }

    fn new_with_bright_color(size: u16, height: u16, c: Color) -> Self {
        Firework {
            size,
            height,
            spawn_point: calculate_spawn_point(size, height),
            color: c,
            trail_color: c,
            is_bright: true,
            is_trail_bright: true,
            ..Default::default()
        }
    }

    fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    fn spawn_point(mut self, point: (u16, u16)) -> Self {
        self.spawn_point = point;
        self
    }

    fn count(mut self, c: FireworkCount) -> Self {
        self.count = c;
        self
    }

    fn color(mut self, col: Color) -> Self {
        self.color = col;
        self
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_bright(mut self, is_bright: bool) -> Self {
        self.is_bright = is_bright;
        self
    }

    fn fade_delay(mut self, delay: f32) -> Self {
        self.fade_delay = delay;
        self
    }

    fn trail_length(mut self, len: u16) -> Self {
        self.trail_length = len;
        self
    }

    fn trail_color(mut self, color: Color) -> Self {
        self.trail_color = color;
        self
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_trail_bright(mut self, is_bright: bool) -> Self {
        self.is_trail_bright = is_bright;
        self
    }
}

impl Default for Firework {
    fn default() -> Self {
        Firework {
            size: 7,
            height: 15,
            spawn_point: (4, 8),
            count: FireworkCount::Finite(1),
            color: Color::Yellow,
            is_bright: false,
            fade_delay: 2.0,
            trail_length: 1,
            trail_color: Color::Yellow,
            is_trail_bright: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Firework, FireworkCount};

    #[test]
    fn test_count_into() {
        let x: u16 = 200;
        let c: FireworkCount = x.into();
        assert_eq!(c, FireworkCount::Finite(x));
    }

    #[test]
    fn test_new_firework() {
        let f1 = Firework {
            size: 20,
            height: 40,
            spawn_point: (10, 40),
            count: 1.into(),
            color: super::Color::Yellow,
            is_bright: false,
            fade_delay: 2.0,
            trail_length: 1,
            trail_color: super::Color::Yellow,
            is_trail_bright: false,
        };
        let f2 = Firework::new(20, 40);
        assert_eq!(f1, f2);
    }

    #[test]
    fn test_default_firework() {
        let f1: Firework = Firework {
            size: 7,
            height: 15,
            ..Default::default()
        };
        assert_eq!(f1, Firework::default());
    }
}
