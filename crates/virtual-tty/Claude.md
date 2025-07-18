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

**See**: The comprehensive test suite organized by behavior and stream type for usage patterns.

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
- **Thread Safety**: Concurrent access to terminal buffer
- **ANSI Support**: Handles common terminal escape sequences (cursor movement, clearing)
- **Modular Structure**: Organized into focused modules for maintainability

### Type-Safe ANSI Parsing
- **Tokenized Parsing**: Converts input into structured tokens
- **Error Handling**: Proper error types for validation failures
- **Command Validation**: All ANSI commands are validated before execution
- **Fallback Support**: Legacy parser maintained for backward compatibility

### Testing Strategy
- **Unit Tests**: Platform-independent tests using direct VirtualTty API calls
- **Isolated Testing**: Focus on VirtualTty behavior in isolation
- **Deterministic**: Consistent behavior across different platforms
- **Snapshot Testing**: Uses inline snapshots for automatic test management

### Unit Testing Guidelines
- Use direct commands: `tty.stdout_write()`, `tty.stderr_write()`, `tty.get_snapshot()`
- Test ANSI escape sequences: cursor movement, clearing, colors
- Test terminal behavior: line wrapping, scrolling, buffer management
- Avoid process spawning or external commands
- Focus on pure VirtualTty functionality

### Snapshot Testing with Insta
- **Inline Snapshots**: Use `insta::assert_snapshot!(snapshot, @"")` for all snapshot assertions
- **Format**: Output includes visual `\n` markers for line endings
- **Benefits**: Automatic snapshot management, visual diffs, easy updates

## Test File Organization

The test suite is organized by **behavior** and **stream type** using a 3-tier structure:

### Test Categories by Behavior

1. **Complex Scenarios** - Multi-command sequences and advanced interactions
2. **Cursor Navigation** - Relative and absolute cursor movement tests  
3. **Cursor State** - Cursor position tracking and scrolling behavior
4. **Screen Editing** - Screen manipulation operations (clearing, etc.)

### Test Files by Stream Type

Each behavior category has 3 corresponding test files:

#### Stdout Tests (4 files)
- `complex_scenarios_tests.rs` - Multi-command sequences using `stdout_write()`
- `cursor_navigation_tests.rs` - Cursor movement tests using `stdout_write()`
- `cursor_state_tests.rs` - Cursor position tracking using `stdout_write()`
- `screen_editing_tests.rs` - Screen manipulation using `stdout_write()`

#### Stderr Tests (4 files)
- `stderr_complex_scenarios_tests.rs` - Multi-command sequences using `stderr_write()`
- `stderr_cursor_navigation_tests.rs` - Cursor movement tests using `stderr_write()`
- `stderr_cursor_state_tests.rs` - Cursor position tracking using `stderr_write()`
- `stderr_screen_editing_tests.rs` - Screen manipulation using `stderr_write()`

#### Mixed Tests (4 files)
- `mixed_complex_scenarios_tests.rs` - Multi-command sequences with mixed `stdout_write()` and `stderr_write()`
- `mixed_cursor_navigation_tests.rs` - Cursor movement with mixed streams
- `mixed_cursor_state_tests.rs` - Cursor position tracking with mixed streams
- `mixed_screen_editing_tests.rs` - Screen manipulation with mixed streams

### Guidelines for New Tests

When adding new tests, choose the appropriate file based on:

1. **Behavior**: What terminal functionality are you testing?
   - **Complex scenarios**: Multi-step sequences, real-world patterns
   - **Cursor navigation**: Movement commands (A, B, C, D, H, f)
   - **Cursor state**: Position tracking, scrolling, boundaries
   - **Screen editing**: Clearing operations (J, K), buffer manipulation

2. **Stream Type**: Which write methods are used?
   - **Stdout only**: Use `[behavior]_tests.rs`
   - **Stderr only**: Use `stderr_[behavior]_tests.rs`
   - **Mixed streams**: Use `mixed_[behavior]_tests.rs`

### Example Test Patterns

```rust
// Stdout test
#[test]
fn test_feature_name() {
    let mut tty = VirtualTty::new(width, height);
    tty.stdout_write("content");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @"");
}

// Mixed test
#[test]
fn test_mixed_feature_name() {
    let mut tty = VirtualTty::new(width, height);
    tty.stdout_write("stdout content");
    tty.stderr_write("stderr content");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @"");
}
```


For full project context, see `../../Claude.md`.