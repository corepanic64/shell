#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut command = String::from("");
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        let c1 = String::from(&command);
        let c2 = String::from("exit");
        if c1.trim() == c2.trim() {
            break;
        } else {
            println!("{}: command not found", command.trim())
        }
    }
}
