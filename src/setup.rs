use crate::display::board::print_board_with_selector;
use std::usize;

use crate::display::{choice::Choice, number_input::NumberInput, refresh_display};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use tgg::crossword::{CrosswordBox, CrosswordBoxValue};

pub struct Config {
    pub width: i32,
    pub height: i32,
    pub use_english_words: bool,
    pub use_latin_words: bool,
    pub board: Vec<Vec<CrosswordBox>>,
}

impl Config {
    pub fn new(width: i32, height: i32) -> Config {
        let board: Vec<Vec<CrosswordBox>> =
            vec![
                vec![
                    CrosswordBox::new(0, CrosswordBoxValue::Empty)
                        .expect("Somehow faild to crate default box");
                    width as usize
                ];
                height as usize
            ];
        Config {
            width,
            height,
            use_english_words: true,
            use_latin_words: false,
            board,
        }
    }
}

pub fn ask_for_config() -> Config {
    let width = NumberInput::new().message("Width: ").min(3).max(50).ask();
    let height = NumberInput::new().message("Height: ").min(3).max(50).ask();

    let mut config = Config::new(width, height);

    // ask if the grid should be generated or left empty

    let use_english_words = Choice::new().message("Use english words?").ask();
    let use_latin_words = Choice::new().message("Use latin words?").default_no().ask();

    if use_english_words == false && use_latin_words == false {
        println!("You must pick at least 1 word list.");
        std::process::exit(1);
    }

    config.use_english_words = use_english_words;
    config.use_latin_words = use_latin_words;

    ask_for_board_template(&mut config);
    return config;
}

fn ask_for_board_template(config: &mut Config) {
    let mut selector_x: usize = (config.width / 2) as usize;
    let mut selector_y: usize = (config.height / 2) as usize;

    println!("Press \"Space\" to toggle a box");
    println!("Press \"Enter\" to save");

    let rows = print_board_with_selector(&config.board, Some(selector_x), Some(selector_y));
    let full_clear = rows + 2;
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
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    println!("Quitting...");
                    std::process::exit(0);
                }
                KeyCode::Enter => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    refresh_display(full_clear);
                    break;
                }
                KeyCode::Up => {
                    if selector_y > 0 {
                        selector_y -= 1;
                    } else {
                        selector_y = (config.height - 1) as usize;
                    }
                }
                KeyCode::Down => {
                    if selector_y < (config.height - 1) as usize {
                        selector_y += 1;
                    } else {
                        selector_y = 0;
                    }
                }
                KeyCode::Left => {
                    if selector_x > 0 {
                        selector_x -= 1;
                    } else {
                        selector_x = (config.width - 1) as usize;
                    }
                }
                KeyCode::Right => {
                    if selector_x < (config.width - 1) as usize {
                        selector_x += 1;
                    } else {
                        selector_x = 0;
                    }
                }
                KeyCode::Char(' ') => match config.board[selector_y][selector_x].value {
                    CrosswordBoxValue::Solid => {
                        config.board[selector_y][selector_x].value = CrosswordBoxValue::Empty
                    }
                    _ => config.board[selector_y][selector_x].value = CrosswordBoxValue::Solid,
                },
                KeyCode::Char(char) => {
                    if char >= 'a' && char <= 'z' {
                        config.board[selector_y][selector_x].value =
                            CrosswordBoxValue::Letter(char.to_ascii_uppercase());
                    }
                }
                _ => {}
            },
            _ => {}
        }
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        refresh_display(rows);
        print_board_with_selector(&config.board, Some(selector_x), Some(selector_y));
    }
}
