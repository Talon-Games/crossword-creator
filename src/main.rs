pub mod creator;
pub mod display;
pub mod setup;

use crate::setup::ask_for_config;

fn main() {
    println!("Welcome to crossword creator!");
    println!("Made by: Maksim Straus");
    println!("https://github.com/cqb13");
    println!("Press \"esc\" at any time to exit the program.");
    println!("");
    let config = ask_for_config();
}
