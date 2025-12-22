#[allow(unused_imports)]
use std::io::{self, Write};

enum Action {
    Terminate,
    NoOp,
    Print(String),
    Type(String),
}

fn main() {
    enter_shell();
}

fn enter_shell() {
    loop {
        let mut command = String::from("");
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        match eval_command(&command) {
            Action::Terminate => break,
            Action::NoOp => println!(""),
            Action::Print(str) => println!("{str}"),
            Action::Type(str) => print_builtin_commands(str),
        }
    }
}

fn print_builtin_commands(string: String) {
    match string.trim() {
        "echo" => println!("echo is a shell builtin"),
        "exit" => println!("exit is a shell builtin"),
        "type" => println!("type is a shell builtin"),
        x => println!("{x}: not found"),
    }
}

fn eval_command(input: &str) -> Action {
    let mut cmd = input.split(" ");

    let binary = if let Some(bin) = cmd.next() {
        bin
    } else {
        return Action::NoOp;
    };

    match binary.trim() {
        "exit" => Action::Terminate,
        "echo" => Action::Print(cmd.collect::<Vec<&str>>().join(" ")),
        "type" => Action::Type(cmd.collect::<Vec<&str>>().join(" ")),
        _ => Action::Print(format!("{}: command not found", binary.trim())),
    }
}
