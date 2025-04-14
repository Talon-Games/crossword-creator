use crate::tgg_file::{edit_clues, Clues};
use crate::{
    commands::load_file,
    display::board::{print_board, print_board_with_numbers},
    prints,
};
use tgg::GameData;

pub fn edit(raw_path: String) {
    let file = load_file(raw_path);

    let game_data = match file.get_game_data() {
        GameData::Crossword(game_data) => game_data,
        _ => {
            prints!(
                "[color:bright-red]Error:[color:reset] The game file loaded is not a crossword"
            );
            std::process::exit(1);
        }
    };

    let clues = Clues::new(
        game_data.horizontal_clues.to_owned(),
        game_data.vertical_clues.to_owned(),
    );

    let clues = edit_clues(&game_data.crossword_data, clues);
}
