use crate::{
    display::board::{print_board, print_board_with_numbers},
    prints, Error,
};
use std::{
    io::Read,
    path::{Path, PathBuf},
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

fn load_file(path: PathBuf) -> Result<Vec<u8>, Error> {
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(err) => return Err(Error::IO(err)),
    };

    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer) {
        Ok(_) => {}
        Err(err) => return Err(Error::IO(err)),
    }

    Ok(buffer)
}

fn validate_path(raw_path: String) -> Result<PathBuf, Error> {
    let path = Path::new(&raw_path).to_owned();

    if path.extension().and_then(|ext| ext.to_str()) != Some("tgg") {
        return Err(Error::InvalidInputFileExtension);
    }

    if path.is_dir() {
        return Err(Error::InputPathLeadsToDirectory);
    }

    if !path.exists() {
        return Err(Error::InputFileDoesNotExist);
    }

    return Ok(path);
}
