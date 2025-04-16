use crate::display::choice::Choice;
use crate::display::refresh_display;
use crate::tgg_file::{edit_board, edit_clues, Clues};
use crate::{commands::load_file, prints};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use tgg::crossword::CrosswordData;
use tgg::GameData;

pub fn edit(raw_path: String) {
    let file = load_file(raw_path);

    let mut game_data: CrosswordData = match file.get_game_data() {
        GameData::Crossword(game_data) => game_data,
        _ => {
            prints!(
                "[color:bright-red]Error:[color:reset] The game file loaded is not a crossword"
            );
            std::process::exit(1);
        }
    };

    let mut clues = Clues::new(
        game_data.horizontal_clues.to_owned(),
        game_data.vertical_clues.to_owned(),
    );

    let mut is_board = true;
    if is_board {
        println!("[Edit Board] |  Edit Clues ");
        edit_board(&mut game_data.crossword_data);
    } else {
        println!(" Edit Board  | [Edit Clues]");
    }
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
                    let is_sure = Choice::new()
                        .message("Are you sure you want to continue?")
                        .ask();
                    if is_sure {
                        refresh_display(1);
                        break;
                    } else {
                        continue;
                    }
                }
                KeyCode::Left | KeyCode::Right => {
                    is_board = !is_board;
                }
                _ => {}
            },
            _ => {}
        }
        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        refresh_display(1);
        if is_board {
            println!("[Edit Board] |  Edit Clues ");
            edit_board(&mut game_data.crossword_data);
        } else {
            println!(" Edit Board  | [Edit Clues]");
            edit_clues(&game_data.crossword_data, &mut clues);
        }
    }
}
