pub mod creator;
pub mod display;
pub mod setup;
pub mod tgg_file;

use crate::creator::generate_crossword;
use crate::setup::ask_for_config;
use tgg::crossword::CrosswordBox;
use tgg_file::save;

fn main() {
    // TODO:
    // - add an option to load from existing file
    println!("Welcome to crossword creator!");
    println!("Made by: Maksim Straus");
    println!("https://github.com/cqb13");
    println!("Press \"esc\" at any time to exit the program.");
    println!("");
    let config = ask_for_config();

    let board: Vec<Vec<CrosswordBox>> = generate_crossword(config);

    //TODO: ask for save later
    save(board);
}
