use pathsearch::find_executable_in_path;
use std::env;

impl Command {
    pub fn from_input(input: String) -> Self {
        const BUILTINS: [&str; 4] = ["exit", "echo", "type", "pwd"];
        let args = shlex::split(input.trim()).unwrap();
        if args.is_empty() {
            return Self::EmptyCommand;
        }
        let cmd = &args[0];
        let remaining_args = &args[1..];
        match cmd.as_str() {
            "exit" => {
                let backt = "0".to_string();
                let code_arg = &remaining_args.get(0).unwrap_or(&backt);
                match code_arg.parse::<i32>() {
                    Ok(code) => {
                        return Self::ExitCommand { exit_code: (code) };
                    }
                    Err(_) => {
                        return Self::InvalidCommand {
                            error_text: "This is not valid number".to_string(),
                        };
                    }
                }
            }
            "echo" => {
                return Self::EchoCommand {
                    display_string: remaining_args.join(" "),
                };
            }
            "type" => {
                if BUILTINS.contains(&remaining_args.join("").as_str()) {
                    return Self::TypeCommand {
                        command_name: remaining_args.join(""),
                        command_type: CommandType::Builtin,
                    };
                }
                if let Some(path) = find_executable_in_path(&remaining_args.join("")) {
                    return Self::TypeCommand {
                        command_name: remaining_args.join(""),
                        command_type: CommandType::Executable {
                            path: path.to_string_lossy().to_string(),
                        },
                    };
                } else {
                    return Self::CommandNotFound {
                        invalid_command: remaining_args.join(""),
                    };
                }
            }
            "pwd" => {
                let working_dir = env::current_dir();
                match working_dir {
                    Ok(dir) => {
                        return Self::PwdCommnad {
                            working_dir: dir.to_string_lossy().to_string(),
                        };
                    }
                    Err(e) => {
                        return Self::CommandNotFound {
                            invalid_command: e.to_string(),
                        };
                    }
                }
            }
            _ => {
                if let Some(path) = find_executable_in_path(&cmd) {
                    return Command::CustomCommand {
                        command_name: cmd.to_string(),
                        args: remaining_args.to_vec(),
                    };
                } else {
                    return Command::CommandNotFound {
                        invalid_command: cmd.to_string(),
                    };
                }
            }
        }
    }
}

pub enum Command {
    EchoCommand {
        display_string: String,
    },
    CommandNotFound {
        invalid_command: String,
    },
    PwdCommnad {
        working_dir: String,
    },
    TypeCommand {
        command_name: String,
        command_type: CommandType,
    },
    EmptyCommand,
    ExitCommand {
        exit_code: i32,
    },
    InvalidCommand {
        error_text: String,
    },
    CustomCommand {
        command_name: String,
        args: Vec<String>,
    },
}

pub enum CommandType {
    Builtin,
    Invalid,
    Executable { path: String },
}
