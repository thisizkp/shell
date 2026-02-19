use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;

const BUILTIN_COMMANDS: [&str; 3] = ["type", "echo", "exit"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["type", cmd, ..] => {
                if BUILTIN_COMMANDS.contains(cmd) {
                    println!("{} is a shell builtin", cmd);
                } else {
                    if let Some(path) = find_cmd(&cmd) {
                        println!("{} is {}", cmd, path.display());
                    } else {
                        println!("{}: not found", cmd);
                    }
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

fn find_cmd(cmd: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap();
    for dir in path.split(":") {
        let path = Path::new(dir).join(cmd);
        if path.exists() {
            let is_exec = fs::metadata(&path)
                .map(|m| m.permissions().mode() & 0o111 != 0)
                .unwrap_or(false);

            if is_exec {
                return Some(path);
            }
        }
    }
    return None;
}
