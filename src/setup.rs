use std::usize;

use crate::creator::Square;
use crate::display::{choice::Choice, number_input::NumberInput, refresh_display};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

pub struct Config {
    pub width: i32,
    pub height: i32,
    pub use_english_words: bool,
    pub use_latin_words: bool,
    pub only_words_with_defs: bool,
    pub use_defs_as_hints: bool,
    pub board: Vec<Vec<Square>>,
}

impl Config {
    pub fn new(width: i32, height: i32) -> Config {
        let board: Vec<Vec<Square>> = vec![vec![Square::Empty; width as usize]; height as usize];
        Config {
            width,
            height,
            use_english_words: true,
            use_latin_words: false,
            only_words_with_defs: true,
            use_defs_as_hints: false,
            board,
        }
    }
}

pub fn ask_for_config() -> Config {
    let width = NumberInput::new().message("Width: ").min(1).max(50).ask();
    let height = NumberInput::new().message("Height: ").min(1).max(50).ask();

    let mut config = Config::new(width, height);

    let use_english_words = Choice::new().message("Use english words?").ask();
    let use_latin_words = Choice::new().message("Use latin words?").default_no().ask();

    if use_english_words == false && use_latin_words == false {
        println!("You must pick at least 1 word list.");
        std::process::exit(1);
    }

    let only_words_with_defs = Choice::new()
        .message("Only use words with definitions?")
        .ask();

    if only_words_with_defs == true {
        let use_defs_as_hints = Choice::new()
            .message("Use definitions as hints?")
            .default_no()
            .ask();

        config.use_defs_as_hints = use_defs_as_hints;
    }

    config.use_english_words = use_english_words;
    config.use_latin_words = use_latin_words;
    config.only_words_with_defs = only_words_with_defs;

    ask_for_board_template(&mut config);
    return config;
}

fn ask_for_board_template(config: &mut Config) {
    let mut selector_x: usize = (config.width / 2) as usize;
    let mut selector_y: usize = (config.height / 2) as usize;

    println!("Press enter to toggle a black box");
    println!("Press \"s\" to save");

    let rows = print_board(config, selector_x, selector_y);
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
                KeyCode::Char('s') => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    refresh_display(full_clear);
                    break;
                }
                KeyCode::Up => {
                    if selector_y > 0 {
                        selector_y -= 1;
                    } else {
                        println!("{} {}", selector_y, config.height - 1);
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
                KeyCode::Enter => match config.board[selector_y][selector_x] {
                    Square::Solid => config.board[selector_y][selector_x] = Square::Empty,
                    _ => config.board[selector_y][selector_x] = Square::Solid,
                },
                _ => {}
            },
            _ => {}
        }
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        refresh_display(rows);
        print_board(&config, selector_x, selector_y);
    }
}

fn print_board(config: &Config, selector_x: usize, selector_y: usize) -> i32 {
    let mut rows = 0;
    print_border(config.width as usize);
    rows += 1;

    for (y, row) in config.board.iter().enumerate() {
        print!("|");
        for (x, square) in row.into_iter().enumerate() {
            let display_char = match square {
                Square::Empty => &' ',
                Square::Solid => &'#',
                Square::Letter(letter) => &letter,
            };
            if selector_x == x && selector_y == y {
                print!("[{}]|", display_char);
            } else {
                print!(" {} |", display_char);
            }
        }
        println!();

        rows += 1;
        print_border(config.width as usize);
        rows += 1;
    }

    rows
}

fn print_border(length: usize) {
    print!("+");
    for _ in 0..length {
        print!("---+");
    }
    println!();
}