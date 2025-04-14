use crate::prints;
use crate::Error;
use std::io::Read;
use std::path::{Path, PathBuf};
use tgg::{Game, TggFile};

pub mod create;
pub mod edit;
pub mod inspect;

fn load_file(raw_path: String) -> TggFile {
    let path = match validate_path(raw_path) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let buffer = match load_file_data(path) {
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

    file
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

fn load_file_data(path: PathBuf) -> Result<Vec<u8>, Error> {
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
