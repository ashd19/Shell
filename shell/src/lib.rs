use std::env::{self, current_dir};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

pub const BUILTINS: [&str; 4] = ["echo","type","pwd","history"];

pub fn is_builtin(cmd: &str) -> bool {
    BUILTINS.contains(&cmd)
}

pub fn find_in_path(cmd: &str) -> Option<String> {
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            let full_path = Path::new(dir).join(cmd);
            if full_path.exists() {
                // Check if file has execute permissions
                if let Ok(metadata) = full_path.metadata() {
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 != 0 {
                        return full_path.to_str().map(|s| s.to_string());
                    }
                }
            }
        }
    }
    None
}

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: error: {}", e),
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_supported_commands() {
        assert!(is_builtin("echo"));
        assert!(is_builtin("exit"));
        assert!(is_builtin("type"));
    }

    #[test]
    fn rejects_unknown_commands() {
        assert!(!is_builtin("foo"));
        assert!(!is_builtin(""));
    }
}
