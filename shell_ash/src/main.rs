use shell::{find_in_path, is_builtin, pwd};
use std::{
    io::{self, Write},
    process,
};

fn main() {
    let mut history: Vec<String> = Vec::new();

    loop {
        print!("$ ");
        io::stdout().flush().expect("Error from stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error from readline");
        let input_cmd = input.trim();

        if !input_cmd.is_empty() {
            history.push(input_cmd.to_string());
        }

        match input_cmd {
            "exit 0" | "exit" => process::exit(0),
            "exit 1" => process::exit(1),

            cmd if cmd.starts_with("echo") => {
                println!("{}", &cmd[5..].trim_start());
            }
            cmd if cmd.starts_with("type ") => {
                let command = &cmd[5..].trim();
                if is_builtin(command) {
                    println!("{command} is a shell builtin");
                } else if let Some(path) = find_in_path(command) {
                    println!("{command} is {path}");
                } else {
                    println!("{command}: not found");
                }
            }
            "pwd" => {
                pwd();
            }
            cmd if cmd.starts_with("history") => {
                let parts: Vec<&str> = cmd.split_whitespace().collect();
                if parts.len() == 1 {
                    // Show all history
                    for (i, cmd) in history.iter().enumerate() {
                        println!("{} {cmd}", i + 1);
                    }
                } else if parts.len() == 2 {
                    // Show last n commands
                    if let Ok(n) = parts[1].parse::<usize>() {
                        let start = if history.len() > n {
                            history.len() - n
                        } else {
                            0
                        };
                        for (i, cmd) in history.iter().skip(start).enumerate() {
                            println!("{} {cmd}", start + i + 1);
                        }
                    } else {
                        println!("history: invalid number: {}", parts[1]);
                    }
                } else {
                    println!("history: usage: history [n]");
                }
            }

            _ => println!("{input_cmd}: command not found"),
        }
    }
}
