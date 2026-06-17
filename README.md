# Build Your Own Shell (Rust)

A from-scratch implementation of a POSIX-compliant shell, built as part of the [CodeCrafters](https://codecrafters.io) "Build Your Own Shell" challenge.

## Features

- [x] **REPL Loop:** Interactive command prompt.
- [x] **Command Parsing:** Handles basic command execution and error reporting.
- [x] **Built-ins:**
    - `exit`: Terminate the shell session.
- [ ] **Next Steps:**
    - `echo`: Print arguments to stdout.
    - `type`: Identify command types (built-in vs executable).
    - `pwd`: Print current working directory.
    - Path searching and external command execution.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- `cargo`

### Running the Shell

You can run the shell locally using:

```bash
./your_program.sh
```

Or directly via cargo:

```bash
cargo run
```

### Usage

Once the shell is running, you'll see a `$ ` prompt. You can type commands like:

```bash
$ exit 0
```

## Project Structure

- `src/main.rs`: The main entry point containing the REPL and command processing logic.
- `your_program.sh`: A wrapper script used by CodeCrafters for execution.

## License

This project is for educational purposes as part of the CodeCrafters challenge.
