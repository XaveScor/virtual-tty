use virtual_tty::VirtualTty;

// =============================================================================
// CURSOR POSITION TRACKING - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_cursor_position_tracking_basic() {
    let mut tty = VirtualTty::new(15, 3);
    tty.stdout_write("Hello");
    let (row, col) = tty.get_cursor_position();
    assert_eq!(row, 0);
    assert_eq!(col, 5);

    tty.stderr_write("World");
    let (row, col) = tty.get_cursor_position();
    assert_eq!(row, 0);
    assert_eq!(col, 10);
}

#[test]
fn test_mixed_cursor_position_tracking_comprehensive() {
    let mut tty = VirtualTty::new(15, 5);

    // Initial position
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 0));

    // After writing to stdout
    tty.stdout_write("Hello");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 5));

    // After writing to stderr
    tty.stderr_write(" World");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 11));

    // After newline from stdout
    tty.stdout_write("\n");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    // After cursor movement from stderr
    tty.stderr_write("\x1b[1A"); // Up 1
    tty.stderr_write("\x1b[2C"); // Right 2
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 2));
}

#[test]
fn test_mixed_cursor_alternating_streams() {
    let mut tty = VirtualTty::new(15, 3);

    tty.stdout_write("A");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 1));

    tty.stderr_write("B");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 2));

    tty.stdout_write("C");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 3));

    tty.stderr_write("\n");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    tty.stdout_write("D");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 1));
}

// =============================================================================
// SCROLLING BEHAVIOR - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_cursor_tracking_during_scroll() {
    let mut tty = VirtualTty::new(10, 2);
    tty.stdout_write("Line1\n");
    tty.stderr_write("Line2\n");
    tty.stdout_write("Line3"); // This should scroll
    let (row, col) = tty.get_cursor_position();
    assert_eq!(row, 1); // Should be on last line
    assert_eq!(col, 5); // After "Line3"
}

#[test]
fn test_mixed_relative_movement_after_scroll() {
    let mut tty = VirtualTty::new(10, 2);
    tty.stdout_write("Line1\n");
    tty.stderr_write("Line2\n");
    tty.stdout_write("Line3"); // This should scroll
    tty.stderr_write("\x1b[1A"); // Move up 1 line
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Line2X\nLine3");
}

#[test]
fn test_mixed_scroll_with_cursor_movements() {
    let mut tty = VirtualTty::new(8, 2);
    tty.stdout_write("First\n");
    tty.stderr_write("Second\n");
    tty.stdout_write("Third"); // Scroll
    tty.stderr_write("\x1b[1A"); // Up 1
    tty.stdout_write("\x1b[3D"); // Back 3
    tty.stderr_write("X");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "SeXond\nThird");
}

// =============================================================================
// LINE BOUNDARY BEHAVIOR - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_cursor_at_line_boundaries() {
    let mut tty = VirtualTty::new(5, 3);
    tty.stdout_write("123");
    tty.stderr_write("45678"); // Should wrap to next line
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 3)); // Should be on second line, position 3

    // Test cursor up from wrapped position
    tty.stderr_write("\x1b[1A"); // Up 1
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "123X5\n678");
}

#[test]
fn test_mixed_line_wrapping_behavior() {
    let mut tty = VirtualTty::new(6, 3);
    tty.stdout_write("ABC");
    tty.stderr_write("DEF"); // Fill first line
    tty.stdout_write("GHI"); // Should wrap
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 3));

    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "ABCDEF\nGHI");
}

#[test]
fn test_mixed_newline_handling() {
    let mut tty = VirtualTty::new(10, 4);
    tty.stdout_write("Line1");
    tty.stderr_write("\n");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    tty.stdout_write("Line2");
    tty.stderr_write("\n");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (2, 0));

    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Line1\nLine2");
}

#[test]
fn test_mixed_cursor_state_preservation() {
    let mut tty = VirtualTty::new(12, 3);
    tty.stdout_write("Start");
    tty.stderr_write("\x1b[1;3H"); // Position 1,3
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 2)); // 0-indexed

    tty.stdout_write("Mid");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 5));

    tty.stderr_write("\x1b[2;1H"); // Position 2,1
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    tty.stdout_write("End");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 3));
}

#[test]
fn test_mixed_empty_writes_cursor_state() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Test");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 4));

    tty.stderr_write("");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 4)); // Should remain unchanged

    tty.stdout_write("");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 4)); // Should remain unchanged

    tty.stderr_write("ing");
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 7));
}
