use crate::setup::Config;
use tgg::crossword::{CrosswordBox, CrosswordBoxValue};

pub enum Direction {
    Down,
    Up,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

//TODO: remember to place numbers
pub fn generate_crossword(config: Config) -> Vec<Vec<CrosswordBox>> {
    return config.board;
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
