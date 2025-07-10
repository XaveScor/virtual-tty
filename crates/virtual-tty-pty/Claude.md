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

**See**: The comprehensive test suite in `tests/` directory for usage patterns.

**Documentation**: 
- [TESTING_PATTERNS.md](TESTING_PATTERNS.md) - PTY testing best practices and patterns
- [SNAPSHOT_MANAGEMENT.md](SNAPSHOT_MANAGEMENT.md) - Inline snapshot requirements and management

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

## LLM Guidelines for PTY Testing

### Testing Philosophy
- **Test PTY processing capabilities**: Validate how PTY captures, processes, and applies ANSI sequences from real applications
- **Choose tools based on testing goal**: 
  - Simple commands (`printf`, `echo -e`) for isolated ANSI sequence validation
  - Complex applications (`less`, `vim`, `top`) for real-world scenario testing (scrolling, paging, interactive updates)

### Validation Strategy
- **Absolute assertions over relative comparisons**: Use `assert_eq!(cursor_pos, (2, 5))` instead of `assert!(row > initial_row)`
- **Content preservation testing**: Prove cursor commands don't modify terminal output by comparing snapshots before/after
- **Real-world scenario validation**: Use complex apps to test behaviors like scrolling in `less`, screen updates in `top`, editing in `vim`

### Application Selection Guidelines
- **Scrolling behavior**: Use `less`, `more` to test screen scrolling and buffer management
- **Interactive updates**: Use `top`, `htop` for dynamic content updates
- **Complex cursor movements**: Use `vim` for advanced cursor positioning and screen manipulation
- **Simple ANSI sequences**: Use `printf`, `echo -e` for isolated sequence testing

### Threading and Process Management
- **Always use `wait_for_completion()`**: PTY reader threads must be properly terminated to prevent hanging tests
- **Process lifecycle**: Wait for child processes before checking results

### Test Infrastructure
- **Use fixture files**: Store test content in `tests/fixtures/` instead of temporary files
- **Inline snapshots**: Use `insta::assert_snapshot!` for precise terminal content validation

### Key Dependencies for Testing
- **insta**: Required for snapshot testing in PTY validation tests

## Critical Testing Rules

### ❌ FORBIDDEN: assert!(contains) Testing
- **NEVER use**: `assert!(snapshot.contains("text"))` or similar string matching
- **ALWAYS use**: `insta::assert_snapshot!` for precise content validation
- **Reason**: String matching is fragile and doesn't capture terminal state accurately

### ✅ REQUIRED: Real-world Scenario Testing Only
- **Only test with real applications**: vim, less, bash, ls, cat, etc.
- **No mock applications**: PTY testing validates real terminal behavior
- **Purpose**: Ensure PTY handles actual application output correctly

### ✅ REQUIRED: Small Terminal Sizes for Readable Snapshots
- **Use small terminal sizes**: 40x10, 50x12, or similar compact dimensions
- **Avoid large terminals**: 80x24 creates unreadable snapshots
- **Purpose**: Human-readable snapshots for easy test maintenance and debugging
- **Example**: `PtyAdapter::new(40, 10)` instead of `PtyAdapter::new(80, 24)`

### ✅ REQUIRED: Inline Snapshots Only
- **ALL snapshots MUST be inline**: Use `@"content"` format, no external .snap files
- **Example**: `insta::assert_snapshot!(snapshot, @r"expected content");`
- **Regeneration**: Use `@""` to force regeneration, then `cargo insta accept`
- **Verification**: `find . -name "*.snap" | wc -l` should return 0

### ✅ REQUIRED: Temp Directory Management
- **Use tempfile crate**: `TempDir::new().unwrap()` for OS-agnostic temp directories
- **Copy fixtures to temp**: Never create inline content, always copy from `tests/fixtures/`
- **Set working directory**: Vim runs from temp dir using `.current_dir(temp_dir.path())`
- **Stable filenames**: Use consistent names like `test_visual.txt`, not process IDs

### Implementation Pattern
```rust
use tempfile::TempDir;

fn copy_fixture_to_dir(dir: &Path, fixture_name: &str, target_name: &str) {
    let fixture_path = Path::new("tests/fixtures").join(fixture_name);
    let target_path = dir.join(target_name);
    fs::copy(&fixture_path, &target_path).unwrap();
}

// ✅ CORRECT: TempDir + fixture copying + inline snapshots + stable filenames
#[test]
fn test_vim_operation() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "multiline_content.txt", "test_visual.txt");
    
    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty.spawn_command(
        &mut Command::new("vim")
            .arg("test_visual.txt")
            .current_dir(temp_dir.path()),
    ).unwrap();
    sleep(Duration::from_millis(500));
    
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    First line of text                      \n
    Second line with more content           \n
    ~                                       \n
    -- VISUAL --                            \n
    ");
    
    pty.send_input_str("\x1b:q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
    // TempDir automatically cleans up
}

// ❌ WRONG: Current dir + process IDs + external snapshots + string matching
#[test]
fn test_vim_wrong() {
    let temp_file = format!("test_{}.txt", std::process::id()); // Process ID = unstable
    let mut pty = PtyAdapter::new(80, 24); // Too large
    let mut child = pty.spawn_command(Command::new("vim").arg(&temp_file))?; // Current dir
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot); // External .snap file
    assert!(snapshot.contains("some text")); // String matching
}
```

**Key Principle**: Use the right tool for the testing goal - simple commands for basic ANSI validation, complex applications for real-world PTY behavior testing. Always follow the Critical Testing Rules above for maintainable, reliable tests.

## Additional Resources

- **[TESTING_PATTERNS.md](TESTING_PATTERNS.md)** - Complete guide to PTY testing patterns and best practices
- **[SNAPSHOT_MANAGEMENT.md](SNAPSHOT_MANAGEMENT.md)** - Detailed inline snapshot requirements and management guide

For full project context, see `../../Claude.md`.