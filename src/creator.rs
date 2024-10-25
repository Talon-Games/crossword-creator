use crate::display::board::print_board;
use crate::display::refresh_display;
use crate::setup::Config;

#[derive(Clone, Copy)]
pub enum Square {
    Solid,
    Empty,
    Letter(char),
}

impl Square {
    pub fn value(&self) -> Option<String> {
        match self {
            Square::Letter(letter) => {
                return Some(letter.to_string());
            }
            _ => return None,
        }
    }
}

pub struct BoardDetails {
    pub longest_word: i32,
    pub total_words: i32,
    horizontal_words: i32,
    vertical_words: i32,
}

impl BoardDetails {
    pub fn new(longest_word: i32, horizontal_words: i32, vertical_words: i32) -> BoardDetails {
        BoardDetails {
            longest_word,
            total_words: horizontal_words + vertical_words,
            horizontal_words,
            vertical_words,
        }
    }

    pub fn display(&self) {
        println!(
            "total word: {}\n\thorizontal words: {}\n\tvertical words: {}\nlongest word: {}",
            self.total_words, self.horizontal_words, self.vertical_words, self.longest_word
        );
    }
}

pub struct CrosswordBuilder<'a> {
    board_history: Vec<Vec<Vec<Square>>>,
    word_list: Vec<&'a str>,
}

impl<'a> CrosswordBuilder<'a> {
    pub fn new(board: &Vec<Vec<Square>>, word_list: Vec<&'a str>) -> CrosswordBuilder<'a> {
        CrosswordBuilder {
            board_history: vec![board.to_vec()],
            word_list,
        }
    }

    pub fn is_full(&self) -> bool {
        for row in &self.board_history[self.board_history.len() - 1] {
            for square in row {
                match square {
                    Square::Empty => return false,
                    _ => continue,
                }
            }
        }

        return true;
    }

    pub fn place_word(&mut self) {}

    pub fn display(&self, width: i32) -> i32 {
        print_board(
            &self.board_history[self.board_history.len() - 1],
            width,
            None,
            None,
        )
    }
}

pub fn generate_crossword(config: Config) {
    let board_details = get_board_details(&config.board, config.width, config.height);
    if board_details.total_words == 0 {
        eprintln!("Crossword layout must have room for at least 1 word");
        std::process::exit(1);
    }
    board_details.display();

    let word_list = load_word_list(board_details.longest_word, &config);

    let mut crossword_builder = CrosswordBuilder::new(&config.board, word_list);

    let size = crossword_builder.display(config.width);

    while !crossword_builder.is_full() {
        crossword_builder.place_word();
        refresh_display(size);
        crossword_builder.display(config.width);
        break;
    }
}

fn get_board_details(board: &Vec<Vec<Square>>, width: i32, height: i32) -> BoardDetails {
    let mut longest_word = 0;
    let mut horizontal_words = 0;
    let mut vertical_words = 0;

    // if there are 2 or more consecutive non solid squares in a row or column that are not seperated by solid squares that is a word

    // scan for horizontal words
    for row in board {
        let mut current_word_len = 0;
        for square in row {
            match square {
                Square::Solid => {
                    if current_word_len > 1 {
                        horizontal_words += 1;
                        if current_word_len > longest_word {
                            longest_word = current_word_len;
                        }
                    }
                    current_word_len = 0;
                }
                _ => {
                    current_word_len += 1;
                }
            }
        }
        if current_word_len > 1 {
            horizontal_words += 1;
            if current_word_len > longest_word {
                longest_word = current_word_len;
            }
        }
    }

    // scan for vertical words
    for x in 0..width {
        let mut current_word_len = 0;
        for y in 0..height {
            match board[y as usize][x as usize] {
                Square::Solid => {
                    if current_word_len > 1 {
                        vertical_words += 1;
                        if current_word_len > longest_word {
                            longest_word = current_word_len;
                        }
                    }
                    current_word_len = 0;
                }
                _ => {
                    current_word_len += 1;
                }
            }
        }
        if current_word_len > 1 {
            vertical_words += 1;
            if current_word_len > longest_word {
                longest_word = current_word_len;
            }
        }
    }

    BoardDetails::new(longest_word, horizontal_words, vertical_words)
}

fn load_word_list(longest_word: i32, config: &Config) -> Vec<&str> {
    let mut word_list: Vec<&str> = Vec::new();

    if config.use_english_words {
        let english_words_file = include_str!("./data/words.txt");

        let words: Vec<&str> = english_words_file
            .lines()
            .filter(|line| {
                // only allows words withg definitions if thats what user wants
                if config.only_words_with_defs
                    && line.trim().split("::").collect::<Vec<&str>>()[1]
                        .trim()
                        .len()
                        == 0
                {
                    return false;
                }
                return true;
            })
            .map(|line| line.trim().split("::").collect::<Vec<&str>>()[0].trim())
            .filter(|line| !line.is_empty() && line.len() <= longest_word as usize)
            .collect();

        word_list.extend(words);
    }

    // all Latin words have a definition with them
    if config.use_latin_words {
        let latin_words_file = include_str!("./data/latin_words.txt");

        let words: Vec<&str> = latin_words_file
            .lines()
            .map(|line| line.trim().split("::").collect::<Vec<&str>>()[0].trim())
            .filter(|line| !line.is_empty() && line.len() <= longest_word as usize)
            .collect();

        word_list.extend(words);
    }

    word_list
}
