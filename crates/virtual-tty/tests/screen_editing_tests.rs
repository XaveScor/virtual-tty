use virtual_tty::VirtualTty;

// =============================================================================
// SCREEN MANIPULATION OPERATIONS
// =============================================================================

#[test]
fn test_clear_line_from_cursor() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[3D"); // Move back 3
    tty.stdout_write("123");
    tty.stdout_write("\x1b[K"); // Clear to end of line
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
    tty.stdout_write("Hello\nWorld\nTest");
    tty.stdout_write("\x1b[2A"); // Move up 2 lines
    tty.stdout_write("\x1b[2K"); // Clear entire line
    tty.stdout_write("New");
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
    tty.stdout_write("Line1\nLine2\nLine3\nLine4");
    tty.stdout_write("\x1b[2;3H"); // Move to row 2, col 3
    tty.stdout_write("\x1b[0J"); // Clear from cursor to end of screen
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
    tty.stdout_write("Line1\nLine2\nLine3\nLine4");
    tty.stdout_write("\x1b[2;3H"); // Move to row 2, col 3
    tty.stdout_write("\x1b[J"); // Clear from cursor to end of screen (default)
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
    tty.stdout_write("ABCD\nEFGH\nIJKL");
    tty.stdout_write("\x1b[2;1H"); // Move to row 2, col 1
    tty.stdout_write("\x1b[0J"); // Clear from cursor to end of screen
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
    tty.stdout_write("ABCD\nEFGH\nIJKL");
    tty.stdout_write("\x1b[1;4H"); // Move to row 1, col 4 (end of first line)
    tty.stdout_write("\x1b[0J"); // Clear from cursor to end of screen
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
    tty.stdout_write("Hello\nWorld\nTest");
    tty.stdout_write("\x1b[2;2H"); // Move to row 2, col 2
    tty.stdout_write("\x1b[0J"); // Clear from cursor to end of screen
    tty.stdout_write("X"); // Should write at cursor position
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
    tty.stdout_write("Line1\nLine2\nLine3");
    tty.stdout_write("\x1b[3D"); // Move back 3 on last line
    tty.stdout_write("\x1b[0J"); // Clear from cursor to end of screen
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
    tty.stdout_write("Line1\nLine2\nLine3\nLine4");
    tty.stdout_write("\x1b[2;3H"); // Move to row 2, col 3
    tty.stdout_write("\x1b[1J"); // Clear from beginning to cursor
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
              \n
      ne2     \n
    Line3     \n
    Line4     \n
    ");
}
