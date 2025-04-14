use super::{load_file, validate_path};
use crate::{
    display::board::{print_board, print_board_with_numbers},
    prints,
};
use tgg::{Game, GameData, TggFile};

pub fn inspect(raw_path: String) {
    let path = match validate_path(raw_path) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let buffer = match load_file(path) {
        Ok(buffer) => buffer,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let file = match TggFile::from_bytes(buffer) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    match file.get_game() {
        Game::Crossword => {}
        _ => {
            prints!(
                "[color:bright-red]Error:[color:reset] The game file loaded is not a crossword"
            );
            std::process::exit(1);
        }
    };

    println!("{}", file.get_title());
    println!("{}", file.get_description());
    println!(
        "Created on: {} By: {}",
        file.get_formatted_creation_date(),
        file.get_author()
    );

    match file.get_game_data() {
        GameData::Crossword(data) => {
            print_board_with_numbers(&data.crossword_data);
            print_board(&data.crossword_data);
            println!("Across:");
            for clue in &data.horizontal_clues {
                println!("{}. {}", clue.number, clue.value);
            }
            println!("Down:");
            for clue in &data.vertical_clues {
                println!("{}. {}", clue.number, clue.value);
            }
        }
        _ => {
            prints!(
                "[color:bright-red]Error:[color:reset] The game file loaded is not a crossword"
            );
            std::process::exit(1);
        }
    }
}
