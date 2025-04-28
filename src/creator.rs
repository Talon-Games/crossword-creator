use crate::{display::board::print_board, setup::Config};
use tgg::crossword::{CrosswordBox, CrosswordBoxValue};

pub enum Direction {
    Across,
    Down,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

pub struct WordPlacement {
    pub position: Position,
    pub length: u8,
    pub direction: Direction,
    pub number: u8,
}

impl WordPlacement {
    pub fn new(position: Position, length: u8, direction: Direction, number: u8) -> WordPlacement {
        WordPlacement {
            position,
            length,
            direction,
            number,
        }
    }
}

pub struct WordPlacements {
    pub horizontal: Vec<WordPlacement>,
    pub vertical: Vec<WordPlacement>,
}

impl WordPlacements {
    pub fn new(horizontal: Vec<WordPlacement>, vertical: Vec<WordPlacement>) -> WordPlacements {
        WordPlacements {
            horizontal,
            vertical,
        }
    }
}

pub fn word_counter(board: &mut Vec<Vec<CrosswordBox>>) -> WordPlacements {
    let mut horizontal_word_number: u8 = 1;
    let mut vertical_word_number: u8 = 1;

    let mut horizontal: Vec<WordPlacement> = Vec::new();
    let mut vertical: Vec<WordPlacement> = Vec::new();

    for (y, row) in board.clone().iter().enumerate() {
        for (x, crossword_box) in row.iter().enumerate() {
            match crossword_box.value {
                CrosswordBoxValue::Solid => continue,
                _ => (),
            };

            // Horizontal Words
            if x == 0
                || (board[y][x - 1].value.to_string() == "#"
                    && (x != row.len() - 1 && board[y][x + 1].value.to_string() != "#"))
            {
                if board[y][x].number == 0 {
                    if horizontal_word_number <= vertical_word_number {
                        horizontal_word_number = vertical_word_number;
                    }

                    board[y][x].number = horizontal_word_number
                }

                horizontal_word_number += 1;

                let mut full = true;

                if board[y][x].value.to_string() == " " {
                    full = false;
                }

                let mut word_len = 1;
                let mut offset = 1;
                loop {
                    if x + offset >= row.len() || board[y][x + offset].value.to_string() == "#" {
                        break;
                    }

                    if board[y][x + offset].value.to_string() == " " {
                        full = false;
                        break;
                    }
                    word_len += 1;
                    offset += 1;
                }

                if !full {
                    horizontal.push(WordPlacement::new(
                        Position::new(x, y),
                        word_len,
                        Direction::Across,
                        horizontal_word_number - 1,
                    ));
                }
            }

            // Vertical Words
            if y == 0
                || board[y - 1][x].value.to_string() == "#"
                    && (y != board.len() - 1 && board[y + 1][x].value.to_string() != "#")
            {
                if board[y][x].number == 0 {
                    if vertical_word_number <= horizontal_word_number {
                        vertical_word_number = horizontal_word_number;
                        board[y][x].number = horizontal_word_number;
                    } else {
                        board[y][x].number = vertical_word_number
                    }
                }

                vertical_word_number += 1;

                let mut full = true;

                if board[y][x].value.to_string() == " " {
                    full = false;
                }

                let mut word_len = 1;
                let mut offset = 1;

                loop {
                    if y + offset >= board.len() || board[y + offset][x].value.to_string() == "#" {
                        break;
                    }

                    if board[y + offset][x].value.to_string() == " " {
                        full = false;
                        break;
                    }

                    word_len += 1;
                    offset += 1;
                }

                if !full {
                    vertical.push(WordPlacement::new(
                        Position::new(x, y),
                        word_len,
                        Direction::Down,
                        vertical_word_number - 1,
                    ));
                }
            }
        }
    }

    WordPlacements {
        horizontal,
        vertical,
    }
}

pub fn generate_crossword(mut config: Config) -> Vec<Vec<CrosswordBox>> {
    let word_placements: WordPlacements = word_counter(&mut config.board);

    println!("Horizontal: ");
    for word in word_placements.horizontal {
        println!(
            "{}: ({}, {}), len: {}",
            word.number, word.position.x, word.position.y, word.length
        );
    }

    println!("Vertical: ");
    for word in word_placements.vertical {
        println!(
            "{}: ({}, {}), len: {}",
            word.number, word.position.x, word.position.y, word.length
        );
    }

    print_board(&config.board);

    return config.board;
}

fn load_word_list(longest_word: i32, config: &Config) -> Vec<&str> {
    let mut word_list: Vec<&str> = Vec::new();

    if config.use_english_words {
        let english_words_file = include_str!("./data/english.txt");

        let words: Vec<&str> = english_words_file
            .lines()
            .map(|line| line.trim().split("::").collect::<Vec<&str>>()[0].trim())
            .filter(|line| !line.is_empty() && line.len() <= longest_word as usize)
            .collect();

        word_list.extend(words);
    }

    // all Latin words have a definition with them
    if config.use_latin_words {
        let latin_words_file = include_str!("./data/latin.txt");

        let words: Vec<&str> = latin_words_file
            .lines()
            .map(|line| line.trim().split("::").collect::<Vec<&str>>()[0].trim())
            .filter(|line| !line.is_empty() && line.len() <= longest_word as usize)
            .collect();

        word_list.extend(words);
    }

    word_list
}
