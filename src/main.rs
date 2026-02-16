use std::io::{self, Write};

const SUPPORTED_COMMANDS: [&str; 3] = ["type", "echo", "exit"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["type", cmd, ..] => {
                if SUPPORTED_COMMANDS.contains(cmd) {
                    println!("{} is a shell builtin", cmd);
                } else {
                    println!("{}: not found", cmd);
                }
            }

            ["echo", rest @ ..] => {
                println!("{}", rest.join(" "));
            }
            ["exit", ..] => {
                break;
            }
            [cmd, ..] => {
                println!("{}: command not found", cmd);
            }
            [] => {}
        }
    }
}
