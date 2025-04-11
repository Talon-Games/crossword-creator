use crate::display::refresh_display;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use tgg::crossword::{CrosswordBox, CrosswordBoxValue, CrosswordClue};
use tgg::TggFile;

use crate::display::board::print_board_with_numbers;

use crate::display::text_input::TextInput;

pub struct Clues {
    pub horizontal: Vec<CrosswordClue>,
    pub vertical: Vec<CrosswordClue>,
}

impl Clues {
    pub fn new(horizontal_clues: Vec<CrosswordClue>, vertical_clues: Vec<CrosswordClue>) -> Clues {
        Clues {
            horizontal: horizontal_clues,
            vertical: vertical_clues,
        }
    }

    pub fn display(&mut self, current_clue: usize) {
        println!("Horizontal:");
        for (i, clue) in self.horizontal.iter().enumerate() {
            if i == current_clue {
                print!("[");
            }
            print!(
                "{}. {}",
                clue.number,
                if clue.value == "" { "___" } else { &clue.value }
            );

            if i == current_clue {
                print!("]");
            };

            print!("\n");
        }
        println!("Vertical:");
        for (i, clue) in self.vertical.iter().enumerate() {
            if i + self.horizontal.len() == current_clue {
                print!("[");
            }
            print!(
                "{}. {}",
                clue.number,
                if clue.value == "" { "___" } else { &clue.value }
            );

            if i + self.horizontal.len() == current_clue {
                print!("]");
            };

            print!("\n");
        }
    }
}

pub fn save(board: Vec<Vec<CrosswordBox>>) {
    let clues = get_clues(&board);
    let file = match create_file(board, clues) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    println!("{:?}", file.to_bytes());

    match std::fs::write("./file.tgg", file.to_bytes()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn create_file(board: Vec<Vec<CrosswordBox>>, clues: Clues) -> Result<TggFile, &'static str> {
    let title = "Test Crossword";
    let description = "Test Description";
    let author = "Maksim Straus";

    let file = TggFile::custom_crossword(
        title,
        description,
        author,
        board.len() as u8,
        board[0].len() as u8,
        clues.horizontal,
        clues.vertical,
        board,
    );

    file
}

fn get_clues(board: &Vec<Vec<CrosswordBox>>) -> Clues {
    print_board_with_numbers(&board);
    println!("");
    let mut clues = create_default_clues(board);
    let mut refresh_amount = 2 + clues.horizontal.len() + clues.vertical.len();
    let mut current_clue = 0;
    let mut editing = false;
    println!("↑ ↓: Move Selector | Space: Edit Clue | Enter: Save All");
    clues.display(current_clue);
    loop {
        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        let event = read().unwrap();
        match event {
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    println!("Quitting...");
                    std::process::exit(0);
                }
                KeyCode::Enter => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    refresh_display(refresh_amount as i32 + 1);
                    break;
                }
                KeyCode::Up => {
                    if editing {
                        continue;
                    };
                    if current_clue == 0 {
                        current_clue = clues.horizontal.len() + clues.vertical.len() - 1;
                    } else {
                        current_clue -= 1;
                    }
                }
                KeyCode::Down => {
                    if editing {
                        continue;
                    };
                    if current_clue == clues.horizontal.len() + clues.vertical.len() - 1 {
                        current_clue = 0;
                    } else {
                        current_clue += 1;
                    }
                }
                KeyCode::Char(' ') => {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    let current_text = if current_clue < clues.horizontal.len() {
                        clues.horizontal[current_clue].value.as_str()
                    } else {
                        clues.vertical[current_clue - clues.horizontal.len()]
                            .value
                            .as_str()
                    };

                    let clue = TextInput::new()
                        .message("Clue: ")
                        .set_text(current_text)
                        .ask();

                    if current_clue < clues.horizontal.len() {
                        clues.horizontal[current_clue].value = clue
                    } else {
                        clues.vertical[current_clue - clues.horizontal.len()].value = clue;
                    };
                }
                _ => {}
            },
            _ => {}
        }
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        refresh_display(refresh_amount as i32);
        clues.display(current_clue);
    }

    clues
}

fn create_default_clues(board: &Vec<Vec<CrosswordBox>>) -> Clues {
    let mut horizontal_clues: Vec<CrosswordClue> = Vec::new();
    let mut vertical_clues: Vec<CrosswordClue> = Vec::new();

    for (y, row) in board.iter().enumerate() {
        for (x, crossword_box) in row.iter().enumerate() {
            match crossword_box.value {
                CrosswordBoxValue::Solid => continue,
                _ => (),
            };

            // Horizontal Words
            if x == 0
                || (board[y][x - 1].value.to_string() == "#"
                    && (x != row.len() - 1 && board[y][x + 1].value.to_string() != "#"))
            {
                if board[y][x].number != 0 {
                    horizontal_clues.push(CrosswordClue::new(board[y][x].number, ""));
                }
            }

            // Vertical Words
            if y == 0
                || board[y - 1][x].value.to_string() == "#"
                    && (y != board.len() - 1 && board[y + 1][x].value.to_string() != "#")
            {
                if board[y][x].number != 0 {
                    vertical_clues.push(CrosswordClue::new(board[y][x].number, ""));
                }
            }
        }
    }

    Clues::new(horizontal_clues, vertical_clues)
}
