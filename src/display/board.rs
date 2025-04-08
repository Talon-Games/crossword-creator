use tgg::crossword::CrosswordBox;

pub fn print_board(
    board: &Vec<Vec<CrosswordBox>>,
    selector_x: Option<usize>,
    selector_y: Option<usize>,
) -> i32 {
    let mut rows = 1;
    print_border(board[0].len());

    for (y, row) in board.iter().enumerate() {
        print!("|");
        for (x, square) in row.into_iter().enumerate() {
            if selector_x.is_some() && selector_y.is_some() {
                if selector_x.unwrap() == x && selector_y.unwrap() == y {
                    print!("[{}]|", square.value.to_string());
                } else {
                    print!(" {} |", square.value.to_string());
                }
            } else {
                print!(" {} |", square.value.to_string());
            }
        }
        println!();

        rows += 1;
        print_border(board[0].len());
        rows += 1;
    }

    rows
}

fn print_border(length: usize) {
    print!("+");
    for _ in 0..length {
        print!("---+");
    }
    println!();
}
