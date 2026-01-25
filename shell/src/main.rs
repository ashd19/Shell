use std::{io::{self, Write}, process};
use shell::is_builtin;

fn main() {
    loop {
        print!("$");
        io::stdout().flush().expect("Error from stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error from readline");
        let input_cmd = input.trim();

        match input_cmd {
            "exit 0" | "exit" => process::exit(0),
            "exit 1" => process::exit(1),
            
            
            cmd if is_builtin(cmd) => println!("{cmd} is a builtin command"),



            _ => println!("{input_cmd}: command not found"),
        }
    }
}
