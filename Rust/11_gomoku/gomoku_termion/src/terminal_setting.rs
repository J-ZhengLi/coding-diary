use std::fmt::Display;

use termion::{clear, cursor, style};

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
    /// TODO: Use macro
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type CM = CursorMode;
        let arguments = match self {
            CM::BlinkingBar => format!("{}", cursor::BlinkingBar),
            CM::BlinkingBlock => format!("{}", cursor::BlinkingBlock),
            CM::BlinkingUnderline => format!("{}", cursor::BlinkingUnderline),
            CM::SteadyBar => format!("{}", cursor::SteadyBar),
            CM::SteadyBlock => format!("{}", cursor::SteadyBlock),
            CM::SteadyUnderline => format!("{}", cursor::SteadyUnderline),
            CM::Hidden => format!("{}", cursor::Hide),
        };
        f.write_str(&arguments)
    }
}

#[derive(Clone)]
pub enum TextStyle {
    Blink,
    Bold,
    CrossedOut,
    Faint,
    Framed,
    Italic,
    Invert,
    Underline,
    NoBlinking,
    NoBold,
    NoCrossedOut,
    NoFaint,
    NoInvert,
    NoItalic,
    NoUnderline,
    Reset,
}

impl Display for TextStyle {
    /// TODO: Use macro
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type TS = TextStyle;
        let style = match self {
            TS::Blink => format!("{}", style::Blink),
            TS::Bold => format!("{}", style::Bold),
            TS::CrossedOut => format!("{}", style::CrossedOut),
            TS::Faint => format!("{}", style::Faint),
            TS::Framed => format!("{}", style::Framed),
            TS::Italic => format!("{}", style::Italic),
            TS::Invert => format!("{}", style::Invert),
            TS::Underline => format!("{}", style::Underline),
            TS::NoBlinking => format!("{}", style::NoBlink),
            TS::NoBold => format!("{}", style::NoBold),
            TS::NoCrossedOut => format!("{}", style::NoCrossedOut),
            TS::NoFaint => format!("{}", style::NoFaint),
            TS::NoItalic => format!("{}", style::NoItalic),
            TS::NoInvert => format!("{}", style::NoInvert),
            TS::NoUnderline => format!("{}", style::NoUnderline),
            TS::Reset => format!("{}", style::Reset),
        };
        f.write_str(&style)
    }
}

#[derive(Clone)]
pub struct TermSettings {
    cursor_mode: CursorMode,
    text_style: TextStyle,
    clear_all: bool,
}

impl Display for TermSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}",
            self.cursor_mode,
            self.text_style,
            if self.clear_all {
                format!("{}", clear::All)
            } else {
                String::new()
            }
        ))
    }
}

impl Default for TermSettings {
    fn default() -> Self {
        Self {
            cursor_mode: CursorMode::SteadyBlock,
            text_style: TextStyle::Reset,
            clear_all: false,
        }
    }
}

impl TermSettings {
    pub fn new(clear: bool) -> Self {
        Self {
            clear_all: clear,
            ..Default::default()
        }
    }

    pub fn cursor_mode(&mut self, mode: CursorMode) -> Self {
        self.cursor_mode = mode;
        self.to_owned()
    }

    pub fn text_style(&mut self, style: TextStyle) -> Self {
        self.text_style = style;
        self.to_owned()
    }
}
