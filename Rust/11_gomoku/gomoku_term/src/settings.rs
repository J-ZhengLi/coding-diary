use crossterm::cursor::{CursorShape, DisableBlinking, EnableBlinking, Hide, SetCursorShape};
use std::fmt::Display;

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
