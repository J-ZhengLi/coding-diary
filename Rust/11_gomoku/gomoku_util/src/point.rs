use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl From<(u16, u16)> for Point {
    fn from(val: (u16, u16)) -> Self {
        Self { x: val.0, y: val.1 }
    }
}

impl From<Point> for (u16, u16) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

impl Add for Point {
    type Output = Self;

    /// Saturating add two points
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }
}

fn add_neg(lhs: u16, rhs: i32) -> u16 {
    if rhs < 0 {
        lhs.saturating_sub((-rhs) as u16)
    } else {
        lhs.saturating_add(rhs as u16)
    }
}

fn sub_neg(lhs: u16, rhs: i32) -> u16 {
    if rhs < 0 {
        lhs.saturating_add((-rhs) as u16)
    } else {
        lhs.saturating_sub(rhs as u16)
    }
}

impl Add<(u16, u16)> for Point {
    type Output = Self;

    fn add(self, rhs: (u16, u16)) -> Self::Output {
        let p: Point = rhs.into();
        self + p
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Self;

    /// Saturating add a point and a `(i32, i32)` tuple,
    /// if any of the `i32` value is negative, a corresponding value in Point will be subtracted
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: add_neg(self.x, rhs.0),
            y: add_neg(self.y, rhs.1),
        }
    }
}

impl Sub<(i32, i32)> for Point {
    type Output = Self;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: sub_neg(self.x, rhs.0),
            y: sub_neg(self.y, rhs.1),
        }
    }
}
