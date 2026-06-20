#[allow(unused_imports)]
use std::io::{self, Write};
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
                        _ => println!("{}: not found", cmd_name),
                    }
                }
                _ => println!("{}: not found", trimmed_input),
            }
        }
    }
