use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
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
