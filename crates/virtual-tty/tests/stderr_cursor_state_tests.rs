use virtual_tty::VirtualTty;

// =============================================================================
// CURSOR POSITION TRACKING
// =============================================================================

#[test]
fn test_stderr_cursor_position_tracking_basic() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stderr_write("Hello");
    let (row, col) = tty.get_cursor_position();
    assert_eq!(row, 0);
    assert_eq!(col, 5);
}

#[test]
fn test_stderr_cursor_position_tracking_comprehensive() {
    let mut tty = VirtualTty::new(10, 5);

    // Initial position
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 0));

    // After writing
    tty.stderr_write("Hello");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 5));

    // After newline
    tty.stderr_write("\n");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    // After cursor movement
    tty.stderr_write("\x1b[1A"); // Up 1
    tty.stderr_write("\x1b[2C"); // Right 2
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 2));
}

// =============================================================================
// SCROLLING BEHAVIOR
// =============================================================================

#[test]
fn test_stderr_cursor_tracking_during_scroll() {
    let mut tty = VirtualTty::new(10, 2);
    tty.stderr_write("Line1\nLine2\nLine3"); // This should scroll
    let (row, col) = tty.get_cursor_position();
    assert_eq!(row, 1); // Should be on last line
    assert_eq!(col, 5); // After "Line3"
}

#[test]
fn test_stderr_relative_movement_after_scroll() {
    let mut tty = VirtualTty::new(10, 2);
    tty.stderr_write("Line1\nLine2\nLine3"); // This should scroll
    tty.stderr_write("\x1b[1A"); // Move up 1 line
    tty.stderr_write("X");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Line2X\nLine3");
}

// =============================================================================
// LINE BOUNDARY BEHAVIOR
// =============================================================================

#[test]
fn test_stderr_cursor_at_line_boundaries() {
    let mut tty = VirtualTty::new(5, 3);
    tty.stderr_write("12345678"); // Should wrap to next line
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 3)); // Should be on second line, position 3

    // Test cursor up from wrapped position
    tty.stderr_write("\x1b[1A"); // Up 1
    tty.stderr_write("X");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "123X5\n678");
}
