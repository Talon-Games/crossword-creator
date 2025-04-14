pub mod cli;
pub mod commands;
pub mod creator;
pub mod display;
pub mod error;
pub mod setup;
pub mod styles;
pub mod tgg_file;

use crate::commands::create::create;
use crate::commands::edit::edit;
use crate::commands::inspect::inspect;
pub use crate::error::Error;
use cli::{Cli, CmdOption, Command};

fn main() {
    let cli = Cli::new()
        .with_command(Command::new("help", "Prints help information").with_option(
            CmdOption::new("command", "COMMAND", "The command you want help with").optional(),
        ))
        .with_command(Command::new("version", "Prints version information"))
        .with_command(Command::new(
            "create",
            "Starts the creation process for a new crossword",
        ))
        .with_command(
            Command::new(
                "edit",
                "Loads an existing crossword from a tgg file to edit",
            )
            .with_option(CmdOption::new(
                "input",
                "INPUT",
                "The input path to a tgg file",
            )),
        )
        .with_command(
            Command::new(
                "inspect",
                "Loads an existing crossword from a tgg file and gives all information about it",
            )
            .with_option(CmdOption::new(
                "input",
                "INPUT",
                "The input path to a tgg file",
            )),
        );

    let command = cli.match_commands();

    match command.name.as_str() {
        "help" => {
            let command = command.get_option("command").to_option();
            cli.help(command.as_deref())
        }
        "version" => cli.version(),
        "create" => {
            create();
        }
        "edit" => {
            let path = command.get_option("input").throw_if_none();

            edit(path);
        }
        "inspect" => {
            let path = command.get_option("input").throw_if_none();

            inspect(path);
        }
        _ => cli.help(None),
    }
}
