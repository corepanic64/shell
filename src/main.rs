use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::OnceLock;
mod utils;
use utils::models::Command;
use utils::*;

use crate::utils::models::clean_history;

pub static INITIAL_DIR: OnceLock<PathBuf> = OnceLock::new();

fn main() {
    INITIAL_DIR.set(env::current_dir().unwrap()).unwrap();
    let path_buf = INITIAL_DIR.get().unwrap();
    let path = path_buf.join("src/history.txt");
    clean_history(path).expect("could not clean history");

    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let command = Command::from_input(&input);
        match command {
            Command::ExitCommand { exit_code } => exit_command(exit_code),
            Command::EchoCommand { display_string } => echo_command(display_string),
            Command::InvalidCommand { error_text } => invalid_command(error_text),
            Command::EmptyCommand => {}
            Command::CustomCommand { command_name, args } => custom_command(command_name, args),
            Command::CommandNotFound { invalid_command } => command_not_found(invalid_command),
            Command::PwdCommnad { working_dir } => command_pwd(working_dir),
            Command::TypeCommand {
                command_name,
                command_type,
            } => type_command(command_name, command_type),
            Command::CdCommand { path, is_error } => cd_command(path, is_error),
            Command::HistoryCommand => {}
        }
    }
}
