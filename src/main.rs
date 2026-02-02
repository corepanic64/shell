use pathsearch::find_executable_in_path;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{os::unix::process::CommandExt, process::Command};

enum Action {
    Terminate,
    NoOp,
    Print(String),
    Echo(String),
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
            Action::Print(str) => exec_something(str),
            Action::Echo(str) => println!("{}", str),
            Action::Type(str) => print_builtin_commands(str),
        }
    }
}

fn exec_something(args: String) {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if let Some(&program) = parts.get(0) {
        let args = &parts[1..];
        if let Some(path) = find_executable_in_path(&program) {
            Command::new(path)
                .arg0(program)
                .args(args)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        } else {
            println!("{}: not found", program)
        }
    } else {
        print!("Command not found")
    }
    ()
}

fn print_builtin_commands(string: String) {
    match string.trim() {
        "echo" => println!("echo is a shell builtin"),
        "exit" => println!("exit is a shell builtin"),
        "type" => println!("type is a shell builtin"),
        x => search_files(x),
    }
}

fn search_files(x: &str) {
    if let Some(path) = find_executable_in_path(&x) {
        println!("{} is {}", x, path.to_str().unwrap().to_string())
    } else {
        println!("{}: not found", x)
    }
}

fn eval_command(input: &str) -> Action {
    let input = input.trim();
    let mut cmd = input.split(" ");

    let binary = if let Some(bin) = cmd.next() {
        bin
    } else {
        return Action::NoOp;
    };

    match binary.trim() {
        "exit" => Action::Terminate,
        "echo" => Action::Echo(cmd.collect::<Vec<&str>>().join(" ")),
        "type" => Action::Type(cmd.collect::<Vec<&str>>().join(" ")),
        _ => Action::Print(input.to_string()),
    }
}
