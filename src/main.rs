#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
            print!("$ ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed_input = input.trim();
            
            match trimmed_input {
                "exit 1" => {
                    process::exit(1);
                },
                "exit 0" => {
                    process::exit(0);
                },
                
                 cmd in trimmed_input.split_whitespace() {
                    if cmd == "echo" {
                        println!();
                    } else {
                        println!("{}: command not found", trimmed_input);
                    }
                }
               
                _ => println!("{}: command not found", trimmed_input),
            }     
  }
}
