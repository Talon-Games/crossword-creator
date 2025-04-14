use crate::{
    commands::load_file,
    display::board::{print_board, print_board_with_numbers},
    prints,
};
use tgg::GameData;

pub fn inspect(raw_path: String) {
    let file = load_file(raw_path);

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
