use std::{
    fmt::Display,
    io::{Stdout, Write},
};

use crate::{DetectCursorPos, Goto, RawTerminal};

#[cold]
#[inline(never)]
/// Write message/signal to given raw terminal
pub fn rtwrite<T: Display>(msg: T, out: &mut RawTerminal<Stdout>) {
    write!(out, "{}", msg).expect("Fail to display content");
    out.flush().unwrap_or_default();
}

/// Like `rtwrite` but allowing you to defy where to write messages
pub fn write_at<T: Display>(msg: T, out: &mut RawTerminal<Stdout>, pos: (u16, u16)) {
    if let Ok(orig_cursor_pos) = out.cursor_pos() {
        rtwrite(Goto(pos.0, pos.1), out);
        rtwrite(msg, out);
        rtwrite(Goto(orig_cursor_pos.0, orig_cursor_pos.1), out);
    }
}

/// Like `rtwrite` but this will write messages on top-left of current screen by default
pub fn debug<T: Display>(msg: T, out: &mut RawTerminal<Stdout>) {
    write_at(msg, out, (1, 1));
}
