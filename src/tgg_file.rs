use tgg::crossword::{CrosswordBox, CrosswordBoxValue, CrosswordClue};
use tgg::TggFile;

use crate::display::board::print_board_with_numbers;

pub struct Clues {
    pub horizontal_clues: Vec<CrosswordClue>,
    pub vertical_clues: Vec<CrosswordClue>,
}

impl Clues {
    pub fn new(horizontal_clues: Vec<CrosswordClue>, vertical_clues: Vec<CrosswordClue>) -> Clues {
        Clues {
            horizontal_clues,
            vertical_clues,
        }
    }
}

pub fn save(board: Vec<Vec<CrosswordBox>>) {
    get_clues(&board);
    create_file(board);
}

fn create_file(board: Vec<Vec<CrosswordBox>>) {
    let title = "Test Crossword";
    let description = "Test Description";
    let author = "Maksim Straus";

    let horizontal_clues: Vec<CrosswordClue> = Vec::new();
    let vertical_clues: Vec<CrosswordClue> = Vec::new();

    let file = TggFile::custom_crossword(
        title,
        description,
        author,
        board.len() as u8,
        board[0].len() as u8,
        horizontal_clues,
        vertical_clues,
        board,
    );
}

fn get_clues(board: &Vec<Vec<CrosswordBox>>) {
    println!("");
    print_board_with_numbers(&board);
    let clues = create_default_clues(board);
    println!("Horizontal:");
    for clue in clues.horizontal_clues {
        println!("{}. ___", clue.number);
    }
    println!("Vertical:");
    for clue in clues.vertical_clues {
        println!("{}. ___", clue.number);
    }
    // create ui with numbers words and clues
}

fn create_default_clues(board: &Vec<Vec<CrosswordBox>>) -> Clues {
    let mut horizontal_word_number: u8 = 1;
    let mut vertical_word_number: u8 = 1;

    let mut horizontal_clues: Vec<CrosswordClue> = Vec::new();
    let mut vertical_clues: Vec<CrosswordClue> = Vec::new();

    for (y, row) in board.iter().enumerate() {
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
                if board[y][x].number != 0 {
                    if horizontal_word_number <= vertical_word_number {
                        horizontal_word_number = vertical_word_number;
                    }

                    horizontal_clues.push(CrosswordClue::new(horizontal_word_number, ""));
                }

                horizontal_word_number += 1;
            }

            // Vertical Words
            if y == 0
                || board[y - 1][x].value.to_string() == "#"
                    && (y != board.len() - 1 && board[y + 1][x].value.to_string() != "#")
            {
                if board[y][x].number != 0 {
                    if vertical_word_number <= horizontal_word_number {
                        vertical_word_number = horizontal_word_number;
                    }

                    vertical_clues.push(CrosswordClue::new(vertical_word_number, ""));
                }

                vertical_word_number += 1;
            }
        }
    }

    Clues::new(horizontal_clues, vertical_clues)
}
