use std::cmp::max;
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
        let (rel_x, rel_y) = (max(1, pos.0), max(1, pos.1));
        rtwrite(Goto(rel_x, rel_y), out);
        rtwrite(msg, out);
        rtwrite(Goto(orig_cursor_pos.0, orig_cursor_pos.1), out);
    }
}

/// Write at specified position with center text alignment
pub fn write_at_with_center_alignment<T: Display>(
    msg: T,
    out: &mut RawTerminal<Stdout>,
    pos: (u16, u16),
) {
    let msg_string = format!("{}", &msg);
    let msg_lines = msg_string.lines();
    let msg_line_count = msg_string.lines().count();
    let mut i = 0;
    for line in msg_lines {
        let start_pos_x = pos
            .0
            .saturating_sub((line.len() / 2).try_into().unwrap_or(u16::MAX));
        let start_pos_y: u16 = if i > msg_line_count / 2 {
            (pos.1 as usize).saturating_add(i - msg_line_count / 2)
        } else {
            (pos.1 as usize).saturating_sub(msg_line_count / 2 - i)
        }
        .try_into()
        .unwrap_or(u16::MAX);
        write_at(format!("{}", line), out, (start_pos_x, start_pos_y));
        i += 1;
    }
}

/// Like `rtwrite` but this will write messages on top-left of current screen by default
pub fn debug<T: Display>(msg: T, out: &mut RawTerminal<Stdout>) {
    write_at(format!("[debug: {}]", msg), out, (1, 1));
}
