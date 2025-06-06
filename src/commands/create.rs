use crate::creator::generate_crossword;
use crate::setup::ask_for_config;
use crate::tgg_file::{begin_save, save};
use tgg::crossword::CrosswordBox;

pub fn create() {
    let config = ask_for_config();

    let board: Vec<Vec<CrosswordBox>> = generate_crossword(config);

    let file = begin_save(board);

    save(file);
}
