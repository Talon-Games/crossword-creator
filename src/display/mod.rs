pub mod board;
pub mod choice;
pub mod number_input;
pub mod text_input;

use crossterm::{cursor, terminal, ExecutableCommand};
use std::io;

pub fn refresh_display(lines: i32) {
    for _ in 0..lines {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}
