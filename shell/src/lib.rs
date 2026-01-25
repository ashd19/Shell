pub const BUILTINS: [&str; 3] = ["echo", "exit", "type"];

pub fn is_builtin(cmd: &str) -> bool {
    BUILTINS.contains(&cmd)
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
