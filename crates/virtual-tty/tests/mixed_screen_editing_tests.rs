use virtual_tty::VirtualTty;

// =============================================================================
// SCREEN MANIPULATION OPERATIONS - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_clear_line_from_cursor() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stderr_write("\x1b[3D"); // Move back 3
    tty.stdout_write("123");
    tty.stderr_write("\x1b[K"); // Clear to end of line
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "He123");
}

#[test]
fn test_mixed_clear_screen() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Line1\nLine2\nLine3");
    tty.stderr_write("\x1b[2J"); // Clear screen
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "");
}

#[test]
fn test_mixed_clear_and_write() {
    let mut tty = VirtualTty::new(12, 3);
    tty.stdout_write("Old content");
    tty.stderr_write("\x1b[2J"); // Clear screen
    tty.stdout_write("New content");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "New content");
}

#[test]
fn test_mixed_clear_home_and_write() {
    let mut tty = VirtualTty::new(15, 3);
    tty.stdout_write("Line1\nLine2\nLine3");
    tty.stderr_write("\x1b[H"); // Home
    tty.stdout_write("\x1b[2J"); // Clear screen
    tty.stderr_write("Fresh start");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Fresh start");
}

#[test]
fn test_mixed_line_clearing_operations() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("ABCDEF");
    tty.stderr_write("\x1b[3D"); // Move back 3
    tty.stdout_write("\x1b[K"); // Clear to end of line
    tty.stderr_write("XYZ");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "ABCXYZ");
}

#[test]
fn test_mixed_clear_line_beginning() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stderr_write("\x1b[2D"); // Move back 2
    tty.stdout_write("\x1b[1K"); // Clear from beginning of line to cursor (now implemented)
    tty.stderr_write("X");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "   Xo");
}

#[test]
fn test_mixed_clear_entire_line() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello\nWorld\nTest");
    tty.stderr_write("\x1b[2A"); // Up 2 lines
    tty.stdout_write("\x1b[2K"); // Clear entire line (not implemented, so ignored)
    tty.stderr_write("New");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "HellNew\nWorld\nTest");
}

#[test]
fn test_mixed_multiple_clear_operations() {
    let mut tty = VirtualTty::new(12, 4);
    tty.stdout_write("Line1\nLine2\nLine3\nLine4");
    tty.stderr_write("\x1b[2A"); // Up 2 lines
    tty.stdout_write("\x1b[K"); // Clear to end
    tty.stderr_write("A");
    tty.stdout_write("\x1b[1B"); // Down 1
    tty.stderr_write("\x1b[2K"); // Clear entire line (not implemented, so ignored)
    tty.stdout_write("B");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Line1\nLine2A\nLine3 B\nLine4");
}

#[test]
fn test_mixed_clear_with_cursor_positioning() {
    let mut tty = VirtualTty::new(8, 3);
    tty.stdout_write("ABCD\nEFGH\nIJKL");
    tty.stderr_write("\x1b[2;3H"); // Position 2,3
    tty.stdout_write("\x1b[K"); // Clear to end
    tty.stderr_write("X");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "ABCD\nEFX\nIJKL");
}

#[test]
fn test_mixed_screen_clear_with_home() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Old\nData\nHere");
    tty.stderr_write("\x1b[H"); // Home
    tty.stdout_write("\x1b[2J"); // Clear screen
    tty.stderr_write("Clean");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Clean");
}

#[test]
fn test_mixed_partial_clear_operations() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("123456789");
    tty.stderr_write("\x1b[5D"); // Move back 5
    tty.stdout_write("\x1b[K"); // Clear to end
    tty.stderr_write("ABC");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "1234ABC");
}

#[test]
fn test_mixed_alternating_clear_write() {
    let mut tty = VirtualTty::new(15, 3);
    tty.stdout_write("Initial text");
    tty.stderr_write("\x1b[H"); // Home
    tty.stdout_write("\x1b[K"); // Clear to end of line
    tty.stderr_write("First ");
    tty.stdout_write("\x1b[K"); // Clear to end of line again
    tty.stderr_write("Second");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "First Second");
}

#[test]
fn test_mixed_clear_line_beginning_at_start() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stderr_write("\x1b[5D"); // Move to start of line
    tty.stdout_write("\x1b[1K"); // Clear from beginning to cursor (at position 0)
    tty.stderr_write("X");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Xello");
}

#[test]
fn test_mixed_clear_line_beginning_at_end() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stderr_write("\x1b[1K"); // Clear from beginning to cursor (at end)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "     X");
}

#[test]
fn test_mixed_clear_line_beginning_empty_line() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("\x1b[1K"); // Clear from beginning to cursor on empty line
    tty.stderr_write("Test");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Test");
}
