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
                println!("{}: command not found", input.trim());

            if input.trim() == "exit" {
                std::process::exit(0);
            }
        }
}
