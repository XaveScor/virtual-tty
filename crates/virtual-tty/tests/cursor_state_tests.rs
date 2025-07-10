use std::io::Write;
use virtual_tty::VirtualTty;

// =============================================================================
// CURSOR POSITION TRACKING
// =============================================================================

#[test]
fn test_cursor_position_tracking_basic() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!(row, 0);
    assert_eq!(col, 5);
}

#[test]
fn test_cursor_position_tracking_comprehensive() {
    let mut tty = VirtualTty::new(10, 5);

    // Initial position
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 0));

    // After writing
    write!(tty.stdout, "Hello").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 5));

    // After newline
    write!(tty.stdout, "\n").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    // After cursor movement
    write!(tty.stdout, "\x1b[1A").unwrap(); // Up 1
    write!(tty.stdout, "\x1b[2C").unwrap(); // Right 2
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 2));
}

// =============================================================================
// SCROLLING BEHAVIOR
// =============================================================================

#[test]
fn test_cursor_tracking_during_scroll() {
    let mut tty = VirtualTty::new(10, 2);
    write!(tty.stdout, "Line1\nLine2\nLine3").unwrap(); // This should scroll
    let (row, col) = tty.get_cursor_position();
    assert_eq!(row, 1); // Should be on last line
    assert_eq!(col, 5); // After "Line3"
}

#[test]
fn test_relative_movement_after_scroll() {
    let mut tty = VirtualTty::new(10, 2);
    write!(tty.stdout, "Line1\nLine2\nLine3").unwrap(); // This should scroll
    write!(tty.stdout, "\x1b[1A").unwrap(); // Move up 1 line
    write!(tty.stdout, "X").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line2X    \n
    Line3     \n
    ");
}

// =============================================================================
// LINE BOUNDARY BEHAVIOR
// =============================================================================

#[test]
fn test_cursor_at_line_boundaries() {
    let mut tty = VirtualTty::new(5, 3);
    write!(tty.stdout, "12345678").unwrap(); // Should wrap to next line
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 3)); // Should be on second line, position 3

    // Test cursor up from wrapped position
    write!(tty.stdout, "\x1b[1A").unwrap(); // Up 1
    write!(tty.stdout, "X").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    123X5\n
    678  \n
         \n
    ");
}
