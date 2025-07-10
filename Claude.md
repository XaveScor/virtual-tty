# Virtual TTY Library

## What is VirtualTTY?

VirtualTTY is a Rust library that provides virtual terminal (TTY) emulation for testing and development of terminal applications. It creates an in-memory terminal buffer that can capture and process terminal output, including ANSI escape sequences, without requiring a real terminal or PTY.

## Goals

### Primary Goals
- **Deterministic Testing**: Enable reliable, repeatable testing of CLI applications and terminal-based tools
- **No Real Terminal Dependencies**: Test terminal applications in CI/CD environments without requiring actual terminal sessions
- **ANSI Support**: Handle common terminal escape sequences (cursor movement, colors, clearing) for realistic testing
- **Dual Approach**: Support both pure virtual testing and integration with real processes via PTY

### Key Use Cases
- **Unit Testing CLI Apps**: Test terminal output, input handling, and user interactions programmatically
- **Integration Testing**: Run real commands (vim, less, ls) in controlled virtual environments
- **Development & Debugging**: Capture and analyze terminal output during development
- **Cross-Platform Testing**: Test terminal behavior consistently across different environments

## Architecture

### Crates

#### 1. `virtual-tty` (Core)
**Purpose**: Core virtual terminal emulation for testing terminal applications without dependencies.

**Use when**: Testing CLI applications, need deterministic terminal behavior, capturing terminal output, working with ANSI escape sequences.

**Key API**: `VirtualTty::new(width, height)` → `stdout_write()` → `get_snapshot()`

**If you need to modify this crate**: [Read more here](crates/virtual-tty/Claude.md)

#### 2. `virtual-tty-pty` (PTY Integration)
**Purpose**: PTY integration for testing VirtualTTY against real terminal applications using pseudo-TTY.

**Use when**: Testing VirtualTTY's correctness with real applications (vim, less, bash) instead of mocked apps, validating ANSI escape sequence handling, verifying terminal emulation behavior.

**Key API**: `PtyAdapter::new(width, height)` → `spawn_command(&mut Command)` → `get_snapshot()`

**If you need to modify this crate**: [Read more here](crates/virtual-tty-pty/Claude.md)

## Development Guidelines

### Task Implementation Plan Template
For any task implementation, follow this structured approach:

```
1. [First implementation step]
2. [Second implementation step]
...
N. [Final implementation step]
N+1. Run tests to verify functionality: `cargo test`
N+2. Remove all debug files and debug code
N+3. Fix all linting issues: `cargo clippy`
N+4. Format all touched files: `cargo fmt`
```

**Development Workflow Order**: Implement → Test → Cleanup Debug Files/Code → Lint → Format

This ensures we validate functionality first before applying any style changes, making the development process cleaner and more focused. Tests validate the implementation works correctly before any code style enforcement.

### Handling Time-Dependent Output in Snapshots

**Problem**: Real terminal applications (especially vim) often display time-based messages like "1 second ago", "2 seconds ago", "0 seconds ago" which make tests non-deterministic.

**Solution Pattern**: Use regex to replace time patterns with equivalent spaces to maintain exact formatting:

```rust
use regex::Regex;

let snapshot = pty.get_snapshot();
// Replace any time pattern with same number of spaces
let time_regex = Regex::new(r"\d+\s+seconds?\s+ago").unwrap();
let normalized_snapshot = time_regex.replace_all(&snapshot, |caps: &regex::Captures| {
    " ".repeat(caps.get(0).unwrap().as_str().len())
}).to_string();
insta::assert_snapshot!(normalized_snapshot, @r"...");
```

**Key Principles**:
- **Length Preservation**: Replace with exact same number of spaces to maintain line formatting
- **Comprehensive Pattern**: `\d+\s+seconds?\s+ago` handles "1 second ago", "2 seconds ago", "0 seconds ago", etc.
- **Deterministic Testing**: Removes timing dependency while preserving layout validation
- **Dependency Required**: Add `regex = "1.0"` to `[dev-dependencies]` in `Cargo.toml`

**Examples**:
- "1 second ago" (12 chars) → 12 spaces
- "2 seconds ago" (13 chars) → 13 spaces  
- "15 seconds ago" (14 chars) → 14 spaces