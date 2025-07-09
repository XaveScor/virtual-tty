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

### Code Formatting
For any task that involves modifying code files, you must format all touched files using `cargo fmt` after finishing the task:
```bash
cargo fmt
```