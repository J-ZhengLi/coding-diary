use crossterm::{
    cursor::{CursorShape, DisableBlinking, EnableBlinking, Hide, SetCursorShape},
    queue,
};
use std::{
    fmt::Display,
    io::{stdout, Error, Write},
};

#[derive(Clone)]
pub enum CursorMode {
    BlinkingBar,
    BlinkingBlock,
    BlinkingUnderline,
    SteadyBar,
    SteadyBlock,
    SteadyUnderline,
    Hidden,
}

impl Display for CursorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type CM = CursorMode;
        let arguments = match self {
            CM::BlinkingBar => format!("{}{}", EnableBlinking, SetCursorShape(CursorShape::Line)),
            CM::BlinkingBlock => {
                format!("{}{}", EnableBlinking, SetCursorShape(CursorShape::Block))
            }
            CM::BlinkingUnderline => format!(
                "{}{}",
                EnableBlinking,
                SetCursorShape(CursorShape::UnderScore)
            ),
            CM::SteadyBar => format!("{}{}", DisableBlinking, SetCursorShape(CursorShape::Line)),
            CM::SteadyBlock => format!("{}{}", DisableBlinking, SetCursorShape(CursorShape::Block)),
            CM::SteadyUnderline => format!(
                "{}{}",
                DisableBlinking,
                SetCursorShape(CursorShape::UnderScore)
            ),
            CM::Hidden => format!("{}", Hide),
        };
        f.write_str(&arguments)
    }
}

fn set_cursor(shape: Option<CursorShape>, enable_blinking: bool) -> Result<(), Error> {
    let mut stdout = stdout();
    if shape.is_none() {
        queue!(stdout, Hide)?;
    }
    if enable_blinking {
        queue!(stdout, SetCursorShape(shape.unwrap()), EnableBlinking)?;
    } else {
        queue!(stdout, SetCursorShape(shape.unwrap()), DisableBlinking)?;
    };
    stdout.flush()?;
    Ok(())
}

impl CursorMode {
    pub fn set(&self) -> Result<(), Error> {
        type CM = CursorMode;
        match self {
            CM::BlinkingBar => set_cursor(Some(CursorShape::Line), true),
            CM::BlinkingBlock => set_cursor(Some(CursorShape::Block), true),
            CM::BlinkingUnderline => set_cursor(Some(CursorShape::UnderScore), true),
            CM::SteadyBar => set_cursor(Some(CursorShape::Line), false),
            CM::SteadyBlock => set_cursor(Some(CursorShape::Block), false),
            CM::SteadyUnderline => set_cursor(Some(CursorShape::UnderScore), false),
            CM::Hidden => set_cursor(None, false),
        }
    }
}

#[derive(Clone)]
pub struct TermSettings {
    cursor_mode: CursorMode,
}

impl Display for TermSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.cursor_mode,))
    }
}

impl Default for TermSettings {
    fn default() -> Self {
        Self {
            cursor_mode: CursorMode::SteadyBlock,
        }
    }
}

impl TermSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cursor_mode(&mut self, mode: CursorMode) -> Self {
        self.cursor_mode = mode;
        self.to_owned()
    }
}

pub struct GameSettings {
    pub fps: u16,
    pub board_size: (u16, u16),
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fps: 30,
            board_size: (15, 15),
        }
    }
}
