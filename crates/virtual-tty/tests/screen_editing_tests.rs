use std::io::Write;
use virtual_tty::VirtualTty;

// =============================================================================
// SCREEN MANIPULATION OPERATIONS
// =============================================================================

#[test]
fn test_clear_line_from_cursor() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[3D").unwrap(); // Move back 3
    write!(tty.stdout, "123").unwrap();
    write!(tty.stdout, "\x1b[K").unwrap(); // Clear to end of line
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    He123     \n
              \n
              \n
    ");
}

#[test]
fn test_clear_entire_line() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello\nWorld\nTest").unwrap();
    write!(tty.stdout, "\x1b[2A").unwrap(); // Move up 2 lines
    write!(tty.stdout, "\x1b[2K").unwrap(); // Clear entire line
    write!(tty.stdout, "New").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
        New   \n
    World     \n
    Test      \n
    ");
}

#[test]
fn test_clear_from_cursor_to_end_of_screen() {
    let mut tty = VirtualTty::new(10, 4);
    write!(tty.stdout, "Line1\nLine2\nLine3\nLine4").unwrap();
    write!(tty.stdout, "\x1b[2;3H").unwrap(); // Move to row 2, col 3
    write!(tty.stdout, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
    Li        \n
              \n
              \n
    ");
}

#[test]
fn test_clear_from_cursor_to_end_of_screen_default() {
    let mut tty = VirtualTty::new(10, 4);
    write!(tty.stdout, "Line1\nLine2\nLine3\nLine4").unwrap();
    write!(tty.stdout, "\x1b[2;3H").unwrap(); // Move to row 2, col 3
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
fn test_clear_from_cursor_at_start_of_line() {
    let mut tty = VirtualTty::new(8, 3);
    write!(tty.stdout, "ABCD\nEFGH\nIJKL").unwrap();
    write!(tty.stdout, "\x1b[2;1H").unwrap(); // Move to row 2, col 1
    write!(tty.stdout, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    ABCD    \n
            \n
            \n
    ");
}

#[test]
fn test_clear_from_cursor_at_end_of_line() {
    let mut tty = VirtualTty::new(8, 3);
    write!(tty.stdout, "ABCD\nEFGH\nIJKL").unwrap();
    write!(tty.stdout, "\x1b[1;4H").unwrap(); // Move to row 1, col 4 (end of first line)
    write!(tty.stdout, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    ABC     \n
            \n
            \n
    ");
}

#[test]
fn test_clear_from_cursor_preserves_position() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Hello\nWorld\nTest").unwrap();
    write!(tty.stdout, "\x1b[2;2H").unwrap(); // Move to row 2, col 2
    write!(tty.stdout, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    write!(tty.stdout, "X").unwrap(); // Should write at cursor position
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello     \n
    WX        \n
              \n
    ");
}

#[test]
fn test_clear_from_cursor_on_last_line() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Line1\nLine2\nLine3").unwrap();
    write!(tty.stdout, "\x1b[3D").unwrap(); // Move back 3 on last line
    write!(tty.stdout, "\x1b[0J").unwrap(); // Clear from cursor to end of screen
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
    Line2     \n
    Li        \n
    ");
}

#[test]
fn test_clear_from_beginning_to_cursor_basic() {
    let mut tty = VirtualTty::new(10, 4);
    write!(tty.stdout, "Line1\nLine2\nLine3\nLine4").unwrap();
    write!(tty.stdout, "\x1b[2;3H").unwrap(); // Move to row 2, col 3
    write!(tty.stdout, "\x1b[1J").unwrap(); // Clear from beginning to cursor
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
              \n
      ne2     \n
    Line3     \n
    Line4     \n
    ");
}
