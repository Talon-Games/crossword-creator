use crate::creator::Square;

pub fn print_board(
    board: &Vec<Vec<Square>>,
    width: i32,
    selector_x: Option<usize>,
    selector_y: Option<usize>,
) -> i32 {
    let mut rows = 0;
    print_border(width as usize);
    rows += 1;

    for (y, row) in board.iter().enumerate() {
        print!("|");
        for (x, square) in row.into_iter().enumerate() {
            let display_char = match square {
                Square::Empty => &' ',
                Square::Solid => &'#',
                Square::Letter(letter) => &letter,
            };

            if selector_x.is_some() && selector_y.is_some() {
                if selector_x.unwrap() == x && selector_y.unwrap() == y {
                    print!("[{}]|", display_char);
                } else {
                    print!(" {} |", display_char);
                }
            } else {
                print!(" {} |", display_char);
            }
        }
        println!();

        rows += 1;
        print_border(width as usize);
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
