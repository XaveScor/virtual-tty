# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with the `virtual-tty` crate.

## virtual-tty Crate

**Purpose**: Core virtual terminal emulation for testing terminal applications without dependencies.

**Use this crate when**:
- Testing CLI applications that write to stdout/stderr
- Need deterministic terminal behavior in tests
- Want to capture and analyze terminal output
- Working with ANSI escape sequences in controlled environment

**Key API**: `VirtualTty::new(width, height)` → `stdout_write()` → `get_snapshot()`

**See**: `../../examples/basic_tty.rs` and `../../examples/testing_example.rs` for usage patterns.

## Usage Pattern

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

## Design Decisions

### Architecture
- **No Dependencies**: Pure Rust implementation for maximum portability
- **Thread Safety**: Arc<Mutex<>> for concurrent access to terminal buffer
- **Efficient Buffer**: 2D Vec<Vec<char>> for O(1) character access
- **ANSI Support**: Handles common terminal escape sequences (cursor movement, clearing)

### Testing Strategy
- **Unit Tests**: Platform-independent tests using direct VirtualTty API calls
- **No External Dependencies**: Tests should only call VirtualTty methods directly
- **Isolated Testing**: Focus on VirtualTty behavior in isolation, not integration with external processes
- **CI-Friendly**: Works in any environment without external dependencies
- **Deterministic**: Consistent behavior across different platforms

### Unit Testing Guidelines
- Use direct commands: `tty.stdout_write()`, `tty.stderr_write()`, `tty.get_snapshot()`
- Test ANSI escape sequences: cursor movement, clearing, colors
- Test terminal behavior: line wrapping, scrolling, buffer management
- Avoid process spawning or external commands
- Focus on pure VirtualTty functionality

For full project context, see `../../Claude.md`.