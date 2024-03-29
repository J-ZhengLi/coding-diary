use std::ops::{Add, Sub};

pub trait Sum {
    fn add(&self, rhs: (i32, i32)) -> Self;
}

impl Sum for (u16, u16) {
    fn add(&self, rhs: (i32, i32)) -> (u16, u16) {
        (
            self.0.saturating_add(rhs.0 as u16),
            self.1.saturating_add(rhs.1 as u16),
        )
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl PartialOrd for Point {
    fn ge(&self, other: &Self) -> bool {
        self.x >= other.x && self.y >= other.y
    }
    fn gt(&self, other: &Self) -> bool {
        self.x > other.x && self.y > other.y
    }
    fn le(&self, other: &Self) -> bool {
        self.x <= other.x && self.y <= other.y
    }
    fn lt(&self, other: &Self) -> bool {
        self.x < other.x && self.y < other.y
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let x_cmp = self.x.partial_cmp(&other.x);
        let y_cmp = self.y.partial_cmp(&other.y);
        if x_cmp == y_cmp {
            x_cmp
        } else {
            None
        }
    }
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

impl Sub<Point> for Point {
    type Output = Self;
    fn sub(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

#[cfg(test)]
mod point_test {
    use super::Point;

    #[test]
    fn partial_ord_test() {
        let a = Point { x: 1, y: 2 };
        let b = Point::default();
        assert!(a > b);

        let a = Point { x: 10, y: 4 };
        let b = Point { x: 5, y: 100 };
        // This was specifically set as incomparable, where non of the `<,>,<=,>=` would be true
        assert_eq!(a > b, false);
        assert_eq!(a < b, false);
        assert_eq!(a >= b, false);
        assert_eq!(a <= b, false);

        let a = Point { x: 10, y: 10 };
        let b = Point { x: 11, y: 100 };
        assert!(a <= b);
    }
}
