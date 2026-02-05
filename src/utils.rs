use models::CommandType;
use std::process;
pub mod models;

pub fn exit_command(code: i32) {
    process::exit(code)
}

pub fn invalid_command(error_text: String) {}

pub fn command_pwd(working_dir: String) {
    println!("{working_dir}")
}

pub fn cd_command(path: String, is_error: bool) {
    if is_error {
        println!("cd: {}: No such file or directory", path)
    }
}

pub fn echo_command(display_string: String) {
    println!("{display_string}")
}
pub fn custom_command(command_name: String, args: Vec<String>) {
    let output = std::process::Command::new(command_name).args(args).status();
    match output {
        Ok(_) => {}
        Err(e) => println!("Error in custom command: {e}"),
    }
}
pub fn command_not_found(invalid_command: String) {
    println!("{invalid_command}: not found")
}

pub fn type_command(command_name: String, command_type: CommandType) {
    match command_type {
        CommandType::Builtin => println!("{command_name} is a shell builtin"),
        CommandType::Executable { path } => println!("{command_name} is {path}"),
        CommandType::Invalid => println!("{command_name}: not found"),
    }
}
