//! # Make static background
//! A background has buildings, or maybe other stuffs,
//! but for now, it will only has buildings, different kind of buildings.
#![allow(dead_code)]

use crate::{write, RawTerminal, Stdout};
use rand::{thread_rng, Rng};
use std::fmt::{Debug, Display};
use termion::cursor::{DetectCursorPos, Goto};

#[derive(PartialEq)]
pub enum UnicodeElement {
    Roof,
    ThickRoof,
    Wall,
    ThickWall,
    LeftWallAndRoof,
    LeftWallAndThickRoof,
    LeftThickWallAndRoof,
    LeftThickWallAndThickRoof,
    RightWallAndRoof,
    RightWallAndThickRoof,
    RightThickWallAndRoof,
    RightThickWallAndThickRoof,
    LargeBox,
    LargeHollowBox,
    Box,
    HollowBox,
    Rectangle,
    HollowRectangle,
    Circle,
    HollowCircle,
}

impl From<UnicodeElement> for char {
    fn from(item: UnicodeElement) -> Self {
        match item {
            UnicodeElement::Roof => '\u{2500}',                       // ─
            UnicodeElement::ThickRoof => '\u{2501}',                  // ━
            UnicodeElement::Wall => '\u{2502}',                       // │
            UnicodeElement::ThickWall => '\u{2503}',                  // ┃
            UnicodeElement::LeftWallAndRoof => '\u{250C}',            // ┌
            UnicodeElement::LeftWallAndThickRoof => '\u{250D}',       // ┍
            UnicodeElement::LeftThickWallAndRoof => '\u{250E}',       // ┎
            UnicodeElement::LeftThickWallAndThickRoof => '\u{250F}',  // ┏
            UnicodeElement::RightWallAndRoof => '\u{2510}',           // ┐
            UnicodeElement::RightWallAndThickRoof => '\u{2511}',      // ┑
            UnicodeElement::RightThickWallAndRoof => '\u{2512}',      // ┒
            UnicodeElement::RightThickWallAndThickRoof => '\u{2513}', // ┓
            UnicodeElement::LargeBox => '\u{25A0}',                   // ■
            UnicodeElement::LargeHollowBox => '\u{25A1}',             // □
            UnicodeElement::Box => '\u{25AA}',                        // ▪
            UnicodeElement::HollowBox => '\u{25AB}',                  // ▫
            UnicodeElement::Rectangle => '\u{25AC}',                  // ▬
            UnicodeElement::HollowRectangle => '\u{25AD}',            // ▭
            UnicodeElement::Circle => '\u{25CF}',                     // ●
            UnicodeElement::HollowCircle => '\u{25CB}',               // ○
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BuildingShape {
    Rectangular,
    Sphere,
    //Triangle,
    TriangleTop,
}

#[derive(Debug, PartialEq)]
pub enum BuildingLightMode {
    On,
    Off,
    Random,
}

#[derive(Debug, Clone)]
pub enum BuildingError {
    ImpossibleShapeError,
    InternalError,
}

impl Display for BuildingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::ImpossibleShapeError => {
                f.write_str("Unable to initialize such building because of incorrect dimension.")
            }
            Self::InternalError => f.write_str(
                "Error when constructing specified building, this might be caused by \
                other functions, api, etc.
                ",
            ),
        }
    }
}

impl From<std::io::Error> for BuildingError {
    fn from(_: std::io::Error) -> Self {
        Self::InternalError
    }
}

#[derive(Debug)]
pub struct Building {
    height: u8,
    width: u8,
    shape: BuildingShape,
    large_windows: bool,
    light_mode: BuildingLightMode,
}

impl Building {
    pub fn new(height: u8, width: u8) -> Self {
        Building {
            height,
            width,
            shape: BuildingShape::Rectangular,
            large_windows: false,
            light_mode: BuildingLightMode::Off,
        }
    }

    pub fn shape(mut self, s: BuildingShape) -> Self {
        self.shape = s;
        self
    }

    pub fn use_large_windows(mut self, l: bool) -> Self {
        self.large_windows = l;
        self
    }

    pub fn light_mode(mut self, mode: BuildingLightMode) -> Self {
        self.light_mode = mode;
        self
    }

    fn select_window(&self, is_light_off: bool) -> char {
        if self.large_windows {
            if is_light_off {
                char::from(UnicodeElement::LargeHollowBox)
            } else {
                char::from(UnicodeElement::LargeBox)
            }
        } else if is_light_off {
            char::from(UnicodeElement::HollowBox)
        } else {
            char::from(UnicodeElement::Box)
        }
    }

    fn construct_rect_shape(
        &self,
        raw_term: &mut RawTerminal<Stdout>,
    ) -> Result<(), BuildingError> {
        let (start_pos_x, mut start_pos_y) = raw_term.cursor_pos()?;

        // Because a 2D rectangular building will have left and right wall along with a roof.
        // Use one more byte to make sure the values don't trigger overflow
        let width_bound = (self.width as u16) + 1;
        let roof_level = (self.height as u16) + 1;

        // First off, draw the roof level
        let flat_roof = char::from(UnicodeElement::Roof)
            .to_string()
            .repeat(self.width.into());
        start_pos_y -= roof_level;
        let full_roof = format!(
            "{}{}{}{}",
            Goto(start_pos_x, start_pos_y),
            char::from(UnicodeElement::LeftWallAndRoof),
            flat_roof,
            char::from(UnicodeElement::RightWallAndRoof)
        );
        write(raw_term, full_roof);

        // TODO: Find a way to emit this line initialization when BuildingLightMode is not Random
        let mut rng = thread_rng();

        // The rest of building is drawn from top to buttom, left to right
        for h in 1..roof_level {
            write(raw_term, Goto(start_pos_x, start_pos_y + h));
            for w in 0..=width_bound {
                if w == 0 || w == width_bound {
                    write(raw_term, char::from(UnicodeElement::Wall));
                } else {
                    let light_off = match self.light_mode {
                        BuildingLightMode::On => false,
                        BuildingLightMode::Off => true,
                        BuildingLightMode::Random => rng.gen::<bool>(),
                    };
                    let window = self.select_window(light_off);
                    write(raw_term, window);
                }
            }
        }

        Ok(())
    }

    pub fn construct(&self, raw_term: &mut RawTerminal<Stdout>) -> Result<(), BuildingError> {
        match &self.shape {
            BuildingShape::Rectangular => self.construct_rect_shape(raw_term),
            _ => unimplemented!("This building shape is not supported yet!"),
        }
    }
}

impl Default for Building {
    fn default() -> Self {
        Self::new(4, 2)
    }
}

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
            && self.width == other.width
            && self.shape == other.shape
            && self.large_windows == other.large_windows
            && self.light_mode == other.light_mode
    }
}

#[cfg(test)]
mod test {
    use super::{Building, BuildingLightMode, BuildingShape};

    #[test]
    fn test_new_building() {
        let existing_building = Building {
            height: 8,
            width: 4,
            ..Default::default()
        };
        assert_eq!(Building::new(8, 4), existing_building);
    }

    #[test]
    fn test_building_set_shape() {
        let existing_building = Building {
            height: 4,
            width: 4,
            shape: BuildingShape::Sphere,
            ..Default::default()
        };
        assert_eq!(
            Building::new(4, 4).shape(BuildingShape::Sphere),
            existing_building
        );
    }

    #[test]
    fn test_building_builder() {
        let existing_building = Building {
            height: 5,
            width: 4,
            shape: BuildingShape::TriangleTop,
            large_windows: true,
            light_mode: BuildingLightMode::Random,
        };

        let exp_building = Building::new(5, 4)
            .shape(BuildingShape::TriangleTop)
            .use_large_windows(true)
            .light_mode(BuildingLightMode::Random);

        assert_eq!(existing_building, exp_building);
    }
}
