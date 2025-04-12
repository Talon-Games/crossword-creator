#[derive(Debug)]
pub enum Error {
    // -- File Loading
    InvalidInputFileExtension,
    InputPathLeadsToDirectory,
    InputFileDoesNotExist,

    InvalidGame,

    // External
    IO(std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidInputFileExtension => {
                write!(f, "Invalid input file extension, expected .tgg")
            }
            Error::InputPathLeadsToDirectory => {
                write!(f, "Input path leads to a directory")
            }
            Error::InputFileDoesNotExist => {
                write!(f, "Input file does not exist")
            }
            Error::InvalidGame => {
                write!(f, "The game file loaded is not a crossword")
            }
            Error::IO(err) => {
                write!(f, "Failed to open file: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {}
