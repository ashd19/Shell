use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process;

fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_single_quote {
            if ch == '\'' {
                in_single_quote = false;
            } else {
                current.push(ch);
            }
        } else if ch == '\'' {
            in_single_quote = true;
        } else if ch.is_whitespace() {
            if !current.is_empty() {
                args.push(current.clone());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

fn find_in_path(name: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();
    path.split(':').find_map(|dir| {
        let full_path = Path::new(dir).join(name);
        if let Ok(metadata) = fs::metadata(&full_path) {
            if metadata.permissions().mode() & 0o111 != 0 {
                return Some(full_path);
            }
        }
        None
    })
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            continue;
        }

        let parts = parse_args(trimmed_input);
        let command = &parts[0];

        match command.as_str() {
            "exit" => {
                let exit_code = parts.get(1).and_then(|c| c.parse().ok()).unwrap_or(0);
                process::exit(exit_code);
            }
            "echo" => {
                println!("{}", parts[1..].join(" "));
            }
            "cd" => {
                let dir = parts.get(1).map(|s| s.as_str()).unwrap_or("");
                let dir = if dir == "~" {
                    env::var("HOME").unwrap_or_else(|_| dir.to_string())
                } else {
                    dir.to_string()
                };
                if let Err(e) = env::set_current_dir(&dir) {
                    let msg = match e.kind() {
                        std::io::ErrorKind::NotFound => "No such file or directory",
                        _ => "No such file or directory",
                    };
                    println!("cd: {}: {}", dir, msg);
                }
            }
            "pwd" => {
                println!("{}", env::current_dir().unwrap().display());
            }
            "type" => {
                let cmd_name = parts.get(1).map(|s| s.as_str()).unwrap_or("");
                match cmd_name {
                    "echo" | "exit" | "type" | "pwd" | "cd" => println!("{} is a shell builtin", cmd_name),
                    _ => {
                        match find_in_path(cmd_name) {
                            Some(p) => println!("{} is {}", cmd_name, p.display()),
                            None => println!("{}: not found", cmd_name),
                        }
                    }
                }
            }
            _ => {
                let args: Vec<String> = parts[1..].to_vec();
                match find_in_path(command) {
                    Some(p) => {
                        let output = process::Command::new(p)
                            .arg0(command)
                            .args(&args)
                            .output();
                        match output {
                            Ok(out) => {
                                io::stdout().write_all(&out.stdout).unwrap();
                                io::stderr().write_all(&out.stderr).unwrap();
                            }
                            Err(_) => println!("{}: not found", command),
                        }
                    }
                    None => println!("{}: not found", command),
                }
            }
        }
    }
}   
