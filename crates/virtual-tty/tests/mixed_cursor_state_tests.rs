use std::io::Write;
use virtual_tty::VirtualTty;

// =============================================================================
// CURSOR POSITION TRACKING - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_cursor_position_tracking_basic() {
    let mut tty = VirtualTty::new(15, 3);
    write!(tty.stdout, "Hello").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!(row, 0);
    assert_eq!(col, 5);

    write!(tty.stderr, "World").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 5));

    // After writing to stderr
    write!(tty.stderr, " World").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 11));

    // After newline from stdout
    write!(tty.stdout, "\n").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    // After cursor movement from stderr
    write!(tty.stderr, "\x1b[1A").unwrap(); // Up 1
    write!(tty.stderr, "\x1b[2C").unwrap(); // Right 2
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 2));
}

#[test]
fn test_mixed_cursor_alternating_streams() {
    let mut tty = VirtualTty::new(15, 3);

    write!(tty.stdout, "A").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 1));

    write!(tty.stderr, "B").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 2));

    write!(tty.stdout, "C").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 3));

    write!(tty.stderr, "\n").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    write!(tty.stdout, "D").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 1));
}

// =============================================================================
// SCROLLING BEHAVIOR - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_cursor_tracking_during_scroll() {
    let mut tty = VirtualTty::new(10, 2);
    write!(tty.stdout, "Line1\n").unwrap();
    write!(tty.stderr, "Line2\n").unwrap();
    write!(tty.stdout, "Line3").unwrap(); // This should scroll
    let (row, col) = tty.get_cursor_position();
    assert_eq!(row, 1); // Should be on last line
    assert_eq!(col, 5); // After "Line3"
}

#[test]
fn test_mixed_relative_movement_after_scroll() {
    let mut tty = VirtualTty::new(10, 2);
    write!(tty.stdout, "Line1\n").unwrap();
    write!(tty.stderr, "Line2\n").unwrap();
    write!(tty.stdout, "Line3").unwrap(); // This should scroll
    write!(tty.stderr, "\x1b[1A").unwrap(); // Move up 1 line
    write!(tty.stdout, "X").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line2X    \n
    Line3     \n
    ");
}

#[test]
fn test_mixed_scroll_with_cursor_movements() {
    let mut tty = VirtualTty::new(8, 2);
    write!(tty.stdout, "First\n").unwrap();
    write!(tty.stderr, "Second\n").unwrap();
    write!(tty.stdout, "Third").unwrap(); // Scroll
    write!(tty.stderr, "\x1b[1A").unwrap(); // Up 1
    write!(tty.stdout, "\x1b[3D").unwrap(); // Back 3
    write!(tty.stderr, "X").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    SeXond  \n
    Third   \n
    ");
}

// =============================================================================
// LINE BOUNDARY BEHAVIOR - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_cursor_at_line_boundaries() {
    let mut tty = VirtualTty::new(5, 3);
    write!(tty.stdout, "123").unwrap();
    write!(tty.stderr, "45678").unwrap(); // Should wrap to next line
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 3)); // Should be on second line, position 3

    // Test cursor up from wrapped position
    write!(tty.stderr, "\x1b[1A").unwrap(); // Up 1
    write!(tty.stdout, "X").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    123X5\n
    678  \n
         \n
    ");
}

#[test]
fn test_mixed_line_wrapping_behavior() {
    let mut tty = VirtualTty::new(6, 3);
    write!(tty.stdout, "ABC").unwrap();
    write!(tty.stderr, "DEF").unwrap(); // Fill first line
    write!(tty.stdout, "GHI").unwrap(); // Should wrap
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 3));

    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    ABCDEF\n
    GHI   \n
          \n
    ");
}

#[test]
fn test_mixed_newline_handling() {
    let mut tty = VirtualTty::new(10, 4);
    write!(tty.stdout, "Line1").unwrap();
    write!(tty.stderr, "\n").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    write!(tty.stdout, "Line2").unwrap();
    write!(tty.stderr, "\n").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (2, 0));

    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
    Line2     \n
              \n
              \n
    ");
}

#[test]
fn test_mixed_cursor_state_preservation() {
    let mut tty = VirtualTty::new(12, 3);
    write!(tty.stdout, "Start").unwrap();
    write!(tty.stderr, "\x1b[1;3H").unwrap(); // Position 1,3
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 2)); // 0-indexed

    write!(tty.stdout, "Mid").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 5));

    write!(tty.stderr, "\x1b[2;1H").unwrap(); // Position 2,1
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 0));

    write!(tty.stdout, "End").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (1, 3));
}

#[test]
fn test_mixed_empty_writes_cursor_state() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Test").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 4));

    write!(tty.stderr, "").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 4)); // Should remain unchanged

    write!(tty.stdout, "").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 4)); // Should remain unchanged

    write!(tty.stderr, "ing").unwrap();
    let (row, col) = tty.get_cursor_position();
    assert_eq!((row, col), (0, 7));
}
