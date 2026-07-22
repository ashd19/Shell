use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process;

pub fn find_in_path(name: &str) -> Option<PathBuf> {
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

pub fn run_external(command: &str, args: &[String]) {
    match find_in_path(command) {
        Some(p) => {
            let output = process::Command::new(p)
                .arg0(command)
                .args(args)
                .output();
            match output {
                Ok(out) => {
                    io::stdout().write_all(&out.stdout).unwrap();
                    io::stderr().write_all(&out.stderr).unwrap();
                }
                Err(_) => println!("{command}: not found"),
            }
        }
        None => println!("{command}: not found"),
    }
}
