#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut command = String::from("");
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        let cmd = String::from(&command);
        let exit = String::from("exit");
        let echo = String::from("echo");
        let (shell_cmd, rest) = cmd.split_at(4);
        if shell_cmd.trim() == exit.trim() {
            break;
        } else if shell_cmd.trim() == echo.trim() {
            println!("{}", rest.trim())
        } else {
            println!("{}: command not found", command.trim())
        }
    }
}
