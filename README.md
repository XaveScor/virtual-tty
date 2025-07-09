# Virtual TTY

> Test terminal applications without the pain of real terminals

## The Problem

Testing terminal applications is painful. Your tests fail in CI because there's no real terminal. ANSI escape sequences make assertions messy. Terminal behavior varies across platforms. You just want to test your CLI tool!

## The Solution

Virtual TTY gives you a rock-solid, in-memory terminal emulator that just works:

```rust
use virtual_tty::VirtualTty;

#[test]
fn test_my_cli_app() {
    let mut tty = VirtualTty::new(80, 24);
    
    // Your app writes to the terminal
    tty.stdout_write("Hello, World!\n");
    tty.stdout_write("\x1b[32mSuccess!\x1b[0m"); // Green text
    
    // Get clean output for assertions
    let output = tty.get_snapshot();
    assert_eq!(output, 
        "Hello, World!                                                                   \n\
         Success!                                                                        \n\
                                                                                         \n\
         ...                                                                             \n"
    );
}
```

## Why Virtual TTY?

### 🚀 Zero Dependencies
Pure Rust implementation. No system dependencies. No build hassles. It just works.

### 🎯 Deterministic Testing
Same behavior every time, on every platform. Your tests pass locally AND in CI.

### 🧩 Real Terminal Behavior
Handles ANSI escape sequences, cursor movement, screen clearing - everything your users see.

### 🔒 Thread-Safe
Test concurrent terminal operations without fear. Built for modern async applications.

## Real-World Examples

### Testing a Progress Bar

```rust
#[test]
fn test_progress_bar() {
    let mut tty = VirtualTty::new(40, 10);
    
    // Simulate progress updates
    for i in 0..=100 {
        tty.stdout_write(&format!("\rProgress: {:>3}%", i));
    }
    
    let output = tty.get_snapshot();
    // Shows the final state - cursor returns to start of line each time
    assert_eq!(output,
        "Progress: 100%                          \n\
                                                \n\
                                                \n\
                                                \n\
                                                \n\
                                                \n\
                                                \n\
                                                \n\
                                                \n\
                                                \n"
    );
}
```

### Testing Interactive Prompts

```rust
#[test]
fn test_user_prompt() {
    let mut tty = VirtualTty::new(40, 5);
    
    // Display prompt
    tty.stdout_write("Continue? (y/n): ");
    
    // Simulate user input
    tty.send_input("y\n");
    
    // Verify prompt behavior
    let output = tty.get_snapshot();
    assert_eq!(output,
        "Continue? (y/n): y                      \n\
                                                \n\
                                                \n\
                                                \n\
                                                \n"
    );
}
```

### Testing ANSI Colors and Formatting

```rust
#[test]
fn test_colored_output() {
    let mut tty = VirtualTty::new(40, 5);
    
    // Write colored status messages (colors are stripped in output)
    tty.stdout_write("\x1b[31mError: \x1b[0mFile not found\n");
    tty.stdout_write("\x1b[33mWarning: \x1b[0mDeprecated API\n");
    tty.stdout_write("\x1b[32mSuccess: \x1b[0mBuild complete\n");
    
    let output = tty.get_snapshot();
    assert_eq!(output,
        "Error: File not found                   \n\
         Warning: Deprecated API                 \n\
         Success: Build complete                 \n\
                                                \n\
                                                \n"
    );
}
```

### Testing Terminal Navigation

```rust
#[test]
fn test_menu_navigation() {
    let mut tty = VirtualTty::new(20, 6);
    
    // Draw a menu
    tty.stdout_write("Select an option:\n");
    tty.stdout_write("  1. New file\n");
    tty.stdout_write("  2. Open file\n");
    tty.stdout_write("  3. Save file\n");
    
    // Move cursor to option 2 and mark it
    tty.stdout_write("\x1b[3A");      // Up 3 lines
    tty.stdout_write("\x1b[1B");      // Down 1 line
    tty.stdout_write(">");            // Add marker
    
    let output = tty.get_snapshot();
    assert_eq!(output,
        "Select an option:   \n\
         > 2. Open file     \n\
           2. Open file     \n\
           3. Save file     \n\
                            \n\
                            \n"
    );
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
virtual-tty = "0.1.0"
```

## API Overview

```rust
// Create a terminal
let mut tty = VirtualTty::new(width, height);

// Write output
tty.stdout_write("Hello");           // Write to stdout
tty.stderr_write("Error!");          // Write to stderr

// Send input
tty.send_input("user input\n");

// Get terminal state
let output = tty.get_snapshot();     // Get full terminal buffer
let (col, row) = tty.get_cursor_position();  // Get cursor position

// Clear terminal
tty.clear();
```

## Why We Built This

We were tired of:
- Tests failing in CI but passing locally
- Mocking terminal behavior incorrectly
- Complex test setups with real PTYs
- Platform-specific terminal quirks

Virtual TTY eliminates these problems. It's the terminal testing library we wished existed, so we built it.

## Who's Using Virtual TTY?

Perfect for:
- CLI tool developers
- TUI application testing
- Terminal emulator development
- System automation testing
- Educational projects

## Get Started

## License

This project is licensed under either of Apache License, Version 2.0 or MIT license at your option.