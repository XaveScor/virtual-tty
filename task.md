# Virtual TTY Library - Monorepo Structure

## Overview
A modular virtual TTY library split into two focused crates for better testing capabilities and separation of concerns.

## Architecture

### Monorepo Structure
```
virtual-tty/
├── Cargo.toml (workspace)
├── crates/
│   ├── virtual-tty/         # Core terminal emulation
│   └── virtual-tty-pty/     # PTY integration
└── examples/
    ├── basic_tty.rs         # Core VirtualTty usage
    ├── testing_example.rs   # CLI app testing pattern
    ├── pty_ls.rs           # Real command execution
    └── pty_interactive.rs   # Interactive command example
```

### Crates

#### 1. `virtual-tty` (Core)
**Purpose**: Pure virtual terminal emulation for testing applications

**Features**:
- No external dependencies
- Thread-safe buffer management
- ANSI escape sequence support
- Cursor tracking and manipulation
- Screen clearing and line operations

**API**:
```rust
let mut tty = VirtualTty::new(80, 24);
tty.stdout_write("Hello, World!\n");
tty.stderr_write("Error message\n");
let output = tty.get_snapshot();
```

#### 2. `virtual-tty-pty` (PTY Integration)
**Purpose**: Run real processes in virtual terminals using pseudo-TTY

**Features**:
- Real PTY allocation using `libc::openpty`
- Process spawning and management
- Input/output streaming
- Thread-based output reading

**API**:
```rust
let mut pty = PtyAdapter::new(80, 24);
let mut child = pty.spawn_command(&mut Command::new("ls"))?;
child.wait()?;
let output = pty.get_snapshot();
```

## Usage Patterns

### Testing CLI Applications
```rust
use virtual_tty::VirtualTty;

#[test]
fn test_my_app() {
    let mut tty = VirtualTty::new(80, 24);
    my_app.run(&mut tty);
    assert!(tty.get_snapshot().contains("Expected output"));
}
```

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

## Implementation Status

### ✅ Completed Features
- [x] Workspace structure with two focused crates
- [x] Core VirtualTty with ANSI support
- [x] PTY integration with real process execution
- [x] Thread-safe buffer management
- [x] Input/output handling
- [x] Comprehensive test suite
- [x] Working examples for all use cases

### 🔧 Core VirtualTty Features
- [x] Terminal buffer (Vec<Vec<char>>)
- [x] Cursor position tracking
- [x] ANSI escape sequences (cursor movement, clearing)
- [x] Line wrapping and scrolling
- [x] stdout_write/stderr_write methods
- [x] get_snapshot() output
- [x] Window size getters
- [x] Clear operations

### 🔧 PTY Features
- [x] Real PTY allocation
- [x] Process spawning
- [x] Background output reading
- [x] Input sending (send_input_str)
- [x] Proper cleanup on drop

## Design Decisions

### Architecture
- **Separation**: Core TTY logic separate from PTY complexity
- **Dependencies**: Core has no deps, PTY only needs libc
- **Thread Safety**: Arc<Mutex<>> for concurrent access
- **Linux Focus**: LF line endings only, Unix PTY APIs

### Testing Strategy
- **Unit Tests**: Each crate has comprehensive tests
- **Integration Examples**: Real-world usage patterns
- **CI-Friendly**: Core tests don't require PTY

### Performance
- **Minimal Dependencies**: Core is dependency-free
- **Efficient Buffer**: 2D Vec for O(1) character access
- **Background Reading**: Non-blocking PTY output processing

## Package Distribution

When published to crates.io:
- `virtual-tty` - Core testing functionality (most users)
- `virtual-tty-pty` - PTY integration (advanced use cases)

Users import exactly what they need, keeping builds fast and dependencies minimal.