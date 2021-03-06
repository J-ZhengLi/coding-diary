use std::{
    fmt::Display,
    io::{stdout, Write},
};

use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    terminal, Command, ExecutableCommand,
};
use crossterm::{queue, style::Print};

/// Write message/signal to given raw terminal
pub fn write<T: Display>(msg: T) {
    stdout()
        .execute(Print(msg))
        .expect("Unable to print content");
}

/// Execute command, this is more efficient then writing a formatted command
pub fn execute<T: Command>(cmd: T) {
    stdout().execute(cmd).expect("Unable to proccess command");
}

#[cold]
#[inline(never)]
/// Like `rtwrite` but allowing you to defy where to write messages
pub fn write_at<T: Display>(msg: T, pos: (u16, u16)) {
    let mut out = stdout();
    queue!(
        out,
        SavePosition,
        MoveTo(pos.0, pos.1),
        Print(msg),
        RestorePosition
    )
    .expect("Unable to print at given position");
    out.flush().unwrap_or_default();
}

/// Write at specified position with center text alignment
pub fn write_at_with_center_alignment<T: Display>(msg: T, pos: (u16, u16)) {
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
        write_at(format!("{}", line), (start_pos_x, start_pos_y));
        i += 1;
    }
}

pub fn write_at_screen_center<T: Display>(msg: T) {
    if let Ok(size) = terminal::size() {
        write_at_with_center_alignment(msg, (size.0 / 2, size.1 / 2));
    }
}

/// Like `rtwrite` but this will write messages on top-left of current screen by default
pub fn debug<T: Display>(msg: T) {
    write_at(format!("debug: [{}]", msg), (1, 1));
}
