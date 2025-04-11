use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

use crate::display::refresh_display;

pub struct TextInput {
    message: String,
    pub full_size: i32,
    reset_size: i32,
    manual_clear: bool,
    value: String,
}

impl TextInput {
    pub fn new() -> Self {
        TextInput {
            message: String::new(),
            full_size: 2,
            reset_size: 1,
            manual_clear: false,
            value: String::new(),
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn manual_clear(mut self) -> Self {
        self.manual_clear = true;
        self
    }

    pub fn set_text(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn ask(self) -> String {
        println!("{}", self.message);

        let mut current_text = self.value;

        loop {
            println!("> {}", current_text);
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
                    KeyCode::Char(c) => {
                        if c.is_ascii() {
                            current_text.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        current_text.pop();
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        if !self.manual_clear {
                            refresh_display(self.full_size);
                        }
                        return current_text;
                    }
                    _ => {}
                },
                _ => {}
            }

            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            refresh_display(self.reset_size);
        }
    }
}
