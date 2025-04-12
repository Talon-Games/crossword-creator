pub mod cli;
pub mod creator;
pub mod display;
pub mod setup;
pub mod styles;
pub mod tgg_file;

use crate::creator::generate_crossword;
use crate::setup::ask_for_config;
use tgg::crossword::CrosswordBox;
use tgg_file::save;

fn main() {
    // take in CLI options
    // - create: starts up current creation mode
    // - edit: takes in a file path and allows for editing of clues and board
    // - analyze: gives all the information on the crossword
    println!("Welcome to crossword creator!");
    println!("Made by: Maksim Straus");
    println!("https://github.com/cqb13");
    println!("Press \"esc\" at any time to exit the program.");
    println!("");
    let config = ask_for_config();

    let board: Vec<Vec<CrosswordBox>> = generate_crossword(config);

    save(board);
}
