use std::usize;

use crate::setup::Config;

#[derive(Clone, Copy)]
pub enum Square {
    Solid,
    Empty,
    Letter(char),
}

pub struct BoardDetails {
    pub longest_word: i32,
    pub total_words: i32,
    horizontal_words: i32,
    vertical_words: i32,
}

impl BoardDetails {
    pub fn new(longest_word: i32, horizontal_words: i32, vertical_words: i32) -> BoardDetails {
        BoardDetails {
            longest_word,
            total_words: horizontal_words + vertical_words,
            horizontal_words,
            vertical_words,
        }
    }

    pub fn display(&self) {
        println!(
            "total word: {}\n\thorizontal words: {}\n\tvertical words: {}\nlongest word: {}",
            self.total_words, self.horizontal_words, self.vertical_words, self.longest_word
        );
    }
}

pub fn generate_crossword(config: &Config) {
    let board_details = get_board_details(&config.board, config.width, config.height);

    board_details.display();
}

fn get_board_details(board: &Vec<Vec<Square>>, width: i32, height: i32) -> BoardDetails {
    let mut longest_word = 0;
    let mut horizontal_words = 0;
    let mut vertical_words = 0;

    // if there are 2 or more consecutive non solid squares in a row or column that are not seperated by solid squares that is a word

    // scan for horizontal words
    for row in board {
        let mut current_word_len = 0;
        for square in row {
            match square {
                Square::Solid => {
                    if current_word_len > 1 {
                        horizontal_words += 1;
                        if current_word_len > longest_word {
                            longest_word = current_word_len;
                        }
                        current_word_len = 0;
                    }
                }
                _ => {
                    current_word_len += 1;
                }
            }
        }
        if current_word_len > 1 {
            horizontal_words += 1;
            if current_word_len > longest_word {
                longest_word = current_word_len;
            }
        }
    }

    // scan for vertical words
    for x in 0..width {
        let mut current_word_len = 0;
        for y in 0..height {
            match board[y as usize][x as usize] {
                Square::Solid => {
                    if current_word_len > 1 {
                        vertical_words += 1;
                        if current_word_len > longest_word {
                            longest_word = current_word_len;
                        }
                        current_word_len = 0;
                    }
                }
                _ => {
                    current_word_len += 1;
                }
            }
        }
        if current_word_len > 1 {
            vertical_words += 1;
            if current_word_len > longest_word {
                longest_word = current_word_len;
            }
        }
    }

    BoardDetails::new(longest_word, horizontal_words, vertical_words)
}
