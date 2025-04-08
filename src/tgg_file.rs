use tgg::crossword::{CrosswordBox, CrosswordClue};
use tgg::TggFile;

pub fn create_file(width: u8, height: u8, board: Vec<Vec<CrosswordBox>>) {
    let title = "Test Crossword";
    let description = "Test Description";
    let author = "Maksim Straus";

    let horizontal_clues: Vec<CrosswordClue> = Vec::new();
    let vertical_clues: Vec<CrosswordClue> = Vec::new();

    let file = TggFile::custom_crossword(
        title,
        description,
        author,
        width,
        height,
        horizontal_clues,
        vertical_clues,
        board,
    );
}
