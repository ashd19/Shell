use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process;

fn main() {
    loop {
            print!("$ ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed_input = input.trim();
            
            match trimmed_input {
                "exit" => {
                    process::exit(0);
                },
                "exit 1" => {
                    process::exit(1);
                },
                "exit 0" => {
                    process::exit(0);
                },
                cmd if cmd.starts_with("echo") => {
                    println!("{}", &cmd[5..]);
                }
                cmd if cmd.starts_with("type ") => {
                    let cmd_name = cmd[5..].trim();
                    match cmd_name {
                        "echo" | "exit" | "type" => println!("{} is a shell builtin", cmd_name),
                        _ => {
                            let path = env::var("PATH").unwrap_or_default();
                            let found = path.split(':').find_map(|dir| {
                                let full_path = Path::new(dir).join(cmd_name);
                                if let Ok(metadata) = fs::metadata(&full_path) {
                                    if metadata.permissions().mode() & 0o111 != 0 {
                                        return Some(full_path);
                                    }
                                }
                                None
                            });
                            match found {
                                Some(p) => println!("{} is {}", cmd_name, p.display()),
                                None => println!("{}: not found", cmd_name),
                            }
                        }
                    }
                }
                _ => println!("{}: not found", trimmed_input),
            }
        }
    }   
