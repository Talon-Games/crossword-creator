use tgg::crossword::CrosswordBox;

pub fn print_board(board: &Vec<Vec<CrosswordBox>>) -> i32 {
    let mut rows = 1;
    print!("+");
    for _ in 0..board[0].len() {
        print!("---+");
    }
    println!();

    for row in board.iter() {
        print!("|");
        for square in row.iter() {
            print!(" {} |", square.value.to_string());
        }
        println!();

        rows += 1;
        print!("+");
        for _ in 0..board[0].len() {
            print!("---+");
        }
        println!();

        rows += 1;
    }

    rows
}

pub fn print_board_with_selector(
    board: &Vec<Vec<CrosswordBox>>,
    selector_x: Option<usize>,
    selector_y: Option<usize>,
) -> i32 {
    let mut rows = 1;
    print!("+");
    for _ in 0..board[0].len() {
        print!("---+");
    }
    println!();

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
        print!("+");
        for _ in 0..board[0].len() {
            print!("---+");
        }
        println!();

        rows += 1;
    }

    rows
}

pub fn print_board_with_numbers(board: &Vec<Vec<CrosswordBox>>) -> i32 {
    let mut rows = 1;
    print!("+");
    for _ in 0..board[0].len() {
        print!("-----+");
    }
    println!();

    for row in board.iter() {
        print!("|");
        for square in row.iter() {
            print!(
                " {}{} |",
                if square.number != 0 {
                    if square.number.to_string().len() == 1 {
                        format!("0{}", square.number.to_string())
                    } else {
                        square.number.to_string()
                    }
                } else {
                    "  ".to_string()
                },
                square.value.to_string()
            );
        }
        println!();

        rows += 1;
        print!("+");
        for _ in 0..board[0].len() {
            print!("-----+");
        }
        println!();

        rows += 1;
    }

    rows
}
