# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with the `virtual-tty-pty` crate.

## virtual-tty-pty Crate

**Purpose**: PTY integration for testing VirtualTTY against real terminal applications using pseudo-TTY.

**Primary use**: Testing VirtualTTY's correctness by running real applications (vim, less, bash) instead of mocked applications. This validates that VirtualTTY properly handles real-world terminal behavior.

**Use this crate when**:
- Testing VirtualTTY with real terminal applications
- Validating ANSI escape sequence handling against actual programs
- Integration testing VirtualTTY's behavior with interactive commands
- Verifying terminal emulation correctness

**Key API**: `PtyAdapter::new(width, height)` → `spawn_command(&mut Command)` → `get_snapshot()`

**Dependencies**: Requires `libc` for Unix PTY operations. Wraps the core `virtual-tty` crate.

**See**: `../../examples/pty_interactive.rs`, `../../examples/pty_ls.rs`, `../../examples/vim_hjkl.rs` for usage patterns.

## Usage Patterns

### Running Real Commands
```rust
use virtual_tty_pty::PtyAdapter;

let mut pty = PtyAdapter::new(80, 24);
let mut child = pty.spawn_command(&mut Command::new("cat").arg("file.txt"))?;
child.wait()?;
println!("{}", pty.get_snapshot());
```

### Interactive Sessions
```rust
let mut pty = PtyAdapter::new(80, 24);
let mut child = pty.spawn_command(&mut Command::new("bash"))?;
pty.send_input_str("echo 'Hello'\n")?;
pty.send_input_str("exit\n")?;
child.wait()?;
```

## Design Decisions

### Architecture
- **PTY Integration**: Uses `libc::openpty` for real pseudo-TTY allocation
- **Process Management**: Spawns and manages real processes in virtual terminals
- **Background Reading**: Non-blocking PTY output processing via threads
- **Linux Focus**: Unix PTY APIs, LF line endings only

### Testing Strategy
- **Real-world Applications**: Only test with actual terminal applications (vim, less, bash, ls)
- **Platform-specific**: Tests designed for macOS and Linux environments
- **PTY Integration**: Validate VirtualTTY behavior with real PTY processes
- **Real-world Validation**: Ensures VirtualTTY handles actual terminal behavior
- **Example-driven**: Comprehensive examples for common use cases

### Integration Testing Guidelines
- Use real applications: `vim`, `less`, `bash`, `ls`, `cat`, etc.
- Test platform-specific behavior (macOS/Linux PTY differences)
- Focus on PTY integration and process management
- Validate terminal emulation with actual programs
- Test interactive sessions and command sequences
- **Use snapshots**: Capture terminal output with `get_snapshot()` and compare against expected results
- Expect platform dependencies (requires Unix PTY APIs)

For full project context, see `../../Claude.md`.