use tgg::crossword::{CrosswordBox, CrosswordClue};
use tgg::TggFile;

use crate::display::board::{print_board, print_board_with_numbers};

pub fn save(board: Vec<Vec<CrosswordBox>>) {
    get_hints(&board);
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

fn get_hints(board: &Vec<Vec<CrosswordBox>>) {
    print_board_with_numbers(board);

    // create ui with numbers words and clues
}
