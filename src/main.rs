use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process;

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

        let mut parts = trimmed_input.split_whitespace();
        let command = parts.next().unwrap();

        match command {
            "exit" => {
                let exit_code = parts.next().and_then(|c| c.parse().ok()).unwrap_or(0);
                process::exit(exit_code);
            }
            "echo" => {
                let rest: Vec<&str> = parts.collect();
                println!("{}", rest.join(" "));
            }
            "pwd" => {
                println!("{}", env::current_dir().unwrap().display());
            }
            "type" => {
                let cmd_name = parts.next().unwrap_or("");
                match cmd_name {
                    "echo" | "exit" | "type" | "pwd" => println!("{} is a shell builtin", cmd_name),
                    _ => {
                        match find_in_path(cmd_name) {
                            Some(p) => println!("{} is {}", cmd_name, p.display()),
                            None => println!("{}: not found", cmd_name),
                        }
                    }
                }
            }
            _ => {
                let args: Vec<&str> = parts.collect();
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
