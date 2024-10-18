use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

use crate::display::refresh_display;

pub struct Choice {
    message: String,
    default_yes: bool,
    pub full_size: i32,
    manual_clear: bool,
}

impl Choice {
    pub fn new() -> Self {
        Choice {
            message: String::new(),
            default_yes: true,
            full_size: 1,
            manual_clear: false,
        }
    }

    pub fn default_no(mut self) -> Self {
        self.default_yes = false;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn manual_clear(mut self) -> Self {
        self.manual_clear = true;
        self
    }

    pub fn ask(&self) -> bool {
        let default_indicator = if self.default_yes { "[n/Y]" } else { "[y/N]" };
        println!("{} {}:", self.message.trim(), default_indicator);
        loop {
            terminal::enable_raw_mode().expect("Failed to enable raw mode");

            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Esc => {
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Char('y') => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        if !self.manual_clear {
                            refresh_display(self.full_size);
                        }
                        return true;
                    }
                    KeyCode::Char('n') => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        if !self.manual_clear {
                            refresh_display(self.full_size);
                        }
                        return false;
                    }
                    _ => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        if !self.manual_clear {
                            refresh_display(self.full_size);
                        }
                        return self.default_yes;
                    }
                },
                _ => {}
            }
            terminal::disable_raw_mode().expect("Failed to disable raw mode");
        }
    }
}
