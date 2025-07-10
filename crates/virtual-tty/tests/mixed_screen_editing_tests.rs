use std::io::Write;
use virtual_tty::VirtualTty;

// =============================================================================
// SCREEN MANIPULATION OPERATIONS - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_clear_line_from_cursor() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stderr, "\x1b[3D").unwrap(); // Move back 3
    write!(tty.stdout, "123").unwrap();
    write!(tty.stderr, "\x1b[K").unwrap(); // Clear to end of line
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    He123     \n
              \n
              \n
    ");
}

#[test]
fn test_mixed_clear_screen() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Line1\nLine2\nLine3").unwrap();
    write!(tty.stderr, "\x1b[2J").unwrap(); // Clear screen
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    \n
    \n
    \n
    ");
}

#[test]
fn test_mixed_clear_and_write() {
    let mut tty = VirtualTty::new(12, 3);
    write!(tty.stdout, "Old content").unwrap();
    write!(tty.stderr, "\x1b[2J").unwrap(); // Clear screen
    write!(tty.stdout, "New content").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    New content \n
                \n
                \n
    ");
}

#[test]
fn test_mixed_clear_home_and_write() {
    let mut tty = VirtualTty::new(15, 3);
    write!(tty.stdout, "Line1\nLine2\nLine3").unwrap();
    write!(tty.stderr, "\x1b[H").unwrap(); // Home
    write!(tty.stdout, "\x1b[2J").unwrap(); // Clear screen
    write!(tty.stderr, "Fresh start").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Fresh start    \n
                   \n
                   \n
    ");
}

#[test]
fn test_mixed_line_clearing_operations() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "ABCDEF").unwrap();
    write!(tty.stderr, "\x1b[3D").unwrap(); // Move back 3
    write!(tty.stdout, "\x1b[K").unwrap(); // Clear to end of line
    write!(tty.stderr, "XYZ").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    ABCXYZ    \n
              \n
              \n
    ");
}

#[test]
fn test_mixed_clear_line_beginning() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stderr, "\x1b[2D").unwrap(); // Move back 2
    write!(tty.stdout, "\x1b[1K").unwrap(); // Clear from beginning of line to cursor (now implemented)
    write!(tty.stderr, "X").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Xo     \n
           \n
           \n
    ");
}

#[test]
fn test_mixed_clear_entire_line() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello\nWorld\nTest").unwrap();
    write!(tty.stderr, "\x1b[2A").unwrap(); // Up 2 lines
    write!(tty.stdout, "\x1b[2K").unwrap(); // Clear entire line
    write!(tty.stderr, "New").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
        New   \n
    World     \n
    Test      \n
    ");
}

#[test]
fn test_mixed_multiple_clear_operations() {
    let mut tty = VirtualTty::new(12, 4);
    write!(tty.stdout, "Line1\nLine2\nLine3\nLine4").unwrap();
    write!(tty.stderr, "\x1b[2A").unwrap(); // Up 2 lines
    write!(tty.stdout, "\x1b[K").unwrap(); // Clear to end
    write!(tty.stderr, "A").unwrap();
    write!(tty.stdout, "\x1b[1B").unwrap(); // Down 1
    write!(tty.stderr, "\x1b[2K").unwrap(); // Clear entire line
    write!(tty.stdout, "B").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1       \n
    Line2A      \n
          B     \n
    Line4       \n
    ");
}

#[test]
fn test_mixed_clear_with_cursor_positioning() {
    let mut tty = VirtualTty::new(8, 3);
    write!(tty.stdout, "ABCD\nEFGH\nIJKL").unwrap();
    write!(tty.stderr, "\x1b[2;3H").unwrap(); // Position 2,3
    write!(tty.stdout, "\x1b[K").unwrap(); // Clear to end
    write!(tty.stderr, "X").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    ABCD    \n
    EFX     \n
    IJKL    \n
    ");
}

#[test]
fn test_mixed_screen_clear_with_home() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Old\nData\nHere").unwrap();
    write!(tty.stderr, "\x1b[H").unwrap(); // Home
    write!(tty.stdout, "\x1b[2J").unwrap(); // Clear screen
    write!(tty.stderr, "Clean").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Clean     \n
              \n
              \n
    ");
}

#[test]
fn test_mixed_partial_clear_operations() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "123456789").unwrap();
    write!(tty.stderr, "\x1b[5D").unwrap(); // Move back 5
    write!(tty.stdout, "\x1b[K").unwrap(); // Clear to end
    write!(tty.stderr, "ABC").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    1234ABC   \n
              \n
              \n
    ");
}

#[test]
fn test_mixed_alternating_clear_write() {
    let mut tty = VirtualTty::new(15, 3);
    write!(tty.stdout, "Initial text").unwrap();
    write!(tty.stderr, "\x1b[H").unwrap(); // Home
    write!(tty.stdout, "\x1b[K").unwrap(); // Clear to end of line
    write!(tty.stderr, "First ").unwrap();
    write!(tty.stdout, "\x1b[K").unwrap(); // Clear to end of line again
    write!(tty.stderr, "Second").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    First Second   \n
                   \n
                   \n
    ");
}

#[test]
fn test_mixed_clear_line_beginning_at_start() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stderr, "\x1b[5D").unwrap(); // Move to start of line
    write!(tty.stdout, "\x1b[1K").unwrap(); // Clear from beginning to cursor (at position 0)
    write!(tty.stderr, "X").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Xello     \n
              \n
              \n
    ");
}

#[test]
fn test_mixed_clear_line_beginning_at_end() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stderr, "\x1b[1K").unwrap(); // Clear from beginning to cursor (at end)
    write!(tty.stdout, "X").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    X    \n
         \n
         \n
    ");
}

#[test]
fn test_mixed_clear_line_beginning_empty_line() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "\x1b[1K").unwrap(); // Clear from beginning to cursor on empty line
    write!(tty.stderr, "Test").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Test      \n
              \n
              \n
    ");
}

#[test]
fn test_mixed_clear_from_cursor_to_end_of_screen() {
    let mut tty = VirtualTty::new(10, 4);
    write!(tty.stdout, "Line1\nLine2\n").unwrap();
    write!(tty.stderr, "Line3\nLine4").unwrap();
    write!(tty.stdout, "\x1b[2;3H").unwrap(); // Move to row 2, col 3
    write!(tty.stderr, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
    Li        \n
              \n
              \n
    ");
}

#[test]
fn test_mixed_clear_from_cursor_to_end_of_screen_default() {
    let mut tty = VirtualTty::new(10, 4);
    write!(tty.stderr, "Line1\nLine2\n").unwrap();
    write!(tty.stdout, "Line3\nLine4").unwrap();
    write!(tty.stderr, "\x1b[2;3H").unwrap(); // Move to row 2, col 3
    write!(tty.stdout, "\x1b[J").unwrap(); // Clear from cursor to end of screen (default)
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
    Li        \n
              \n
              \n
    ");
}

#[test]
fn test_mixed_clear_from_cursor_at_start_of_line() {
    let mut tty = VirtualTty::new(8, 3);
    write!(tty.stdout, "ABCD\n").unwrap();
    write!(tty.stderr, "EFGH\n").unwrap();
    write!(tty.stdout, "IJKL").unwrap();
    write!(tty.stderr, "\x1b[2;1H").unwrap(); // Move to row 2, col 1
    write!(tty.stdout, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    ABCD    \n
            \n
            \n
    ");
}

#[test]
fn test_mixed_clear_from_cursor_preserves_position() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello\n").unwrap();
    write!(tty.stderr, "World\n").unwrap();
    write!(tty.stdout, "Test").unwrap();
    write!(tty.stderr, "\x1b[2;2H").unwrap(); // Move to row 2, col 2
    write!(tty.stdout, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    write!(tty.stderr, "X").unwrap(); // Should write at cursor position
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello     \n
    WX        \n
              \n
    ");
}

#[test]
fn test_mixed_clear_from_cursor_on_last_line() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stderr, "Line1\n").unwrap();
    write!(tty.stdout, "Line2\n").unwrap();
    write!(tty.stderr, "Line3").unwrap();
    write!(tty.stdout, "\x1b[3D").unwrap(); // Move back 3 on last line
    write!(tty.stderr, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
    Line2     \n
    Li        \n
    ");
}

#[test]
fn test_mixed_clear_from_cursor_with_subsequent_writes() {
    let mut tty = VirtualTty::new(12, 4);
    write!(tty.stdout, "First\nSecond\nThird\nFourth").unwrap();
    write!(tty.stderr, "\x1b[2;4H").unwrap(); // Move to row 2, col 4
    write!(tty.stdout, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    write!(tty.stderr, "NEW").unwrap();
    write!(tty.stdout, "\x1b[1B").unwrap(); // Move down 1 line
    write!(tty.stderr, "MORE").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    First       \n
    SecNEW      \n
          MORE  \n
                \n
    ");
}
