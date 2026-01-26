use std::{io::{self, Write}, process};
use shell::{is_builtin, find_in_path,pwd};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().expect("Error from stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error from readline");
        let input_cmd = input.trim();

        match input_cmd {
            "exit 0" | "exit" => process::exit(0),
            "exit 1" => process::exit(1),
            cmd if cmd.starts_with("echo") =>{
                println!("{}",&cmd[5..].trim_start());
            },
            cmd if cmd.starts_with("type ") => {
                let command = &cmd[5..].trim();
                if is_builtin(command) {
                    println!("{command} is a shell builtin");
                } else if let Some(path) = find_in_path(command) {
                    println!("{command} is {path}");
                } else {
                    println!("{command}: not found");
                }
            },
            "pwd" => {
                    pwd();
            },

            _ => println!("{input_cmd}: command not found"),
        }
    }
}
