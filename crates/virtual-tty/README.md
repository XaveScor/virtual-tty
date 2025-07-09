# virtual-tty

A pure Rust virtual terminal (TTY) emulator for testing terminal applications without requiring a real terminal or PTY.

## Features

- **No Dependencies**: Pure Rust implementation with zero runtime dependencies
- **ANSI Support**: Handles common terminal escape sequences (cursor movement, colors, clearing)
- **Thread Safe**: Safe concurrent access to terminal buffer
- **Deterministic**: Consistent behavior for reliable testing
- **Platform Independent**: Works on all platforms without Unix-specific APIs

## Usage

```rust
use virtual_tty::VirtualTty;

// Create a virtual terminal
let mut tty = VirtualTty::new(80, 24);

// Write to stdout
tty.stdout_write("Hello, world!\n");

// Write ANSI escape sequences
tty.stdout_write("\x1b[31mRed text\x1b[0m\n");

// Get terminal snapshot
let snapshot = tty.get_snapshot();
assert!(snapshot.contains("Red text"));
```

## Testing CLI Applications

```rust
use virtual_tty::VirtualTty;

#[test]
fn test_my_cli_app() {
    let mut tty = VirtualTty::new(80, 24);
    
    // Run your CLI app with the virtual terminal
    my_app.run(&mut tty);
    
    // Verify the output
    let output = tty.get_snapshot();
    assert!(output.contains("Expected output"));
}
```

## ANSI Escape Sequences

Supported sequences include:
- Cursor movement: `ESC[A` (up), `ESC[B` (down), `ESC[C` (right), `ESC[D` (left)
- Cursor positioning: `ESC[H`, `ESC[{row};{col}H`
- Screen clearing: `ESC[J` (clear screen), `ESC[K` (clear line)
- Colors and styles: `ESC[31m` (red), `ESC[1m` (bold), etc.

## License

MIT