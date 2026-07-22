use std::env;

pub enum Builtin {
    Echo,
    Exit,
    Type,
    Pwd,
    Cd,
}

impl Builtin {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "echo" => Some(Self::Echo),
            "exit" => Some(Self::Exit),
            "type" => Some(Self::Type),
            "pwd" => Some(Self::Pwd),
            "cd" => Some(Self::Cd),
            _ => None,
        }
    }

    pub fn names() -> &'static [&'static str] {
        &["echo", "exit", "type", "pwd", "cd"]
    }

    pub fn execute(&self, args: &[String]) {
        match self {
            Self::Echo => println!("{}", args.join(" ")),
            Self::Exit => {
                let code = args.first().and_then(|c| c.parse().ok()).unwrap_or(0);
                std::process::exit(code);
            }
            Self::Type => {
                let cmd = args.first().map(|s| s.as_str()).unwrap_or("");
                if Self::from_name(cmd).is_some() {
                    println!("{cmd} is a shell builtin");
                } else {
                    match crate::command::find_in_path(cmd) {
                        Some(p) => println!("{cmd} is {}", p.display()),
                        None => println!("{cmd}: not found"),
                    }
                }
            }
            Self::Pwd => println!("{}", env::current_dir().unwrap().display()),
            Self::Cd => {
                let dir = args.first().map(|s| s.as_str()).unwrap_or("");
                let dir = if dir == "~" {
                    env::var("HOME").unwrap_or_default()
                } else {
                    dir.to_string()
                };
                if let Err(e) = env::set_current_dir(&dir) {
                    let msg = match e.kind() {
                        std::io::ErrorKind::NotFound => "No such file or directory",
                        _ => "No such file or directory",
                    };
                    println!("cd: {dir}: {msg}");
                }
            }
        }
    }
}
