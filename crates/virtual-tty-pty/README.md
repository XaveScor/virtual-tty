# virtual-tty-pty

PTY integration for virtual-tty - run real processes in virtual terminals for integration testing.

## Features

- **Real Process Execution**: Run actual terminal applications (vim, less, bash) in virtual terminals
- **PTY Integration**: Uses pseudo-TTY for realistic terminal emulation
- **Input/Output Control**: Send input and capture output from running processes
- **Integration Testing**: Test your CLI tools against real terminal applications

## Usage

```rust
use virtual_tty_pty::PtyAdapter;
use std::process::Command;

// Create a PTY adapter
let mut pty = PtyAdapter::new(80, 24)?;

// Spawn a command
let mut child = pty.spawn_command(
    Command::new("echo").arg("Hello from PTY!")
)?;

// Wait for completion
child.wait()?;

// Get the output
println!("{}", pty.get_snapshot());
```

## Interactive Sessions

```rust
use virtual_tty_pty::PtyAdapter;
use std::process::Command;

let mut pty = PtyAdapter::new(80, 24)?;
let mut child = pty.spawn_command(Command::new("bash"))?;

// Send commands
pty.send_input_str("echo 'Hello'\n")?;
pty.send_input_str("ls -la\n")?;
pty.send_input_str("exit\n")?;

child.wait()?;
```

## Platform Support

This crate requires Unix-like systems (Linux, macOS) due to PTY API dependencies.

## Use Cases

- Integration testing with real terminal applications
- Validating ANSI escape sequence handling
- Testing interactive CLI tools
- Automating terminal-based workflows

## License

MIT