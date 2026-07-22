use std::io::{self, Write};

use crate::builtins::Builtin;
use crate::command;
use crate::parser::parse_args;

pub fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        let parts = parse_args(trimmed);
        let command = &parts[0];
        let args: Vec<String> = parts[1..].to_vec();

        match Builtin::from_name(command) {
            Some(builtin) => builtin.execute(&args),
            None => command::run_external(command, &args),
        }
    }
}
