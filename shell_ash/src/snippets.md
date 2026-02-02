## 1] history pagination history <n>

```
cmd if cmd.starts_with("history") => {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let limit = if parts.len() > 1 {
        parts[1].parse::<usize>().ok()
    } else {
        None
    };
    
    let start_index = if let Some(n) = limit {
        history.len().saturating_sub(n)
    } else {
        0
    };
    
    for (i, cmd) in history.iter().enumerate().skip(start_index) {
        println!("  {}  {}", i + 1, cmd);
    }
}
```
