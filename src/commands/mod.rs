use crate::Error;
use std::io::Read;
use std::path::{Path, PathBuf};

pub mod create;
pub mod inspect;

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
