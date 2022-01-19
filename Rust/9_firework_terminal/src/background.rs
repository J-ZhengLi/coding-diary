//! # Make static background
//! A background has buildings, or maybe other stuffs,
//! but for now, it will only has buildings, different kind of buildings.
#![allow(dead_code)]

pub enum BuildingShape {
    Square,
    Sphere,
    Triangle,
    TriangleTop
}

pub struct Building {
    height: u8,
    width: u8,
    shape: BuildingShape
}