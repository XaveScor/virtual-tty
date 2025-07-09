use virtual_tty::VirtualTty;

// =============================================================================
// RELATIVE CURSOR MOVEMENTS
// =============================================================================

// Cursor Up (A command) tests
#[test]
fn test_cursor_up_basic() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Line1\nLine2\nLine3");
    tty.stdout_write("\x1b[1A"); // Move up 1 line
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
    Line2X    \n
    Line3     \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_up_multiple() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Line1\nLine2\nLine3");
    tty.stdout_write("\x1b[2A"); // Move up 2 lines
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1X    \n
    Line2     \n
    Line3     \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_up_no_parameter() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Line1\nLine2");
    tty.stdout_write("\x1b[A"); // Move up 1 line (default)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1X    \n
    Line2     \n
              \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_up_bounds_check() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[10A"); // Try to move up 10 lines (should stop at 0)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    HelloX    \n
              \n
              \n
    ");
}

// Cursor Down (B command) tests
#[test]
fn test_cursor_down_basic() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Line1");
    tty.stdout_write("\x1b[1B"); // Move down 1 line
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
         X    \n
              \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_down_multiple() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Line1");
    tty.stdout_write("\x1b[2B"); // Move down 2 lines
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
              \n
         X    \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_down_no_parameter() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Line1");
    tty.stdout_write("\x1b[B"); // Move down 1 line (default)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line1     \n
         X    \n
              \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_down_bounds_check() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[10B"); // Try to move down 10 lines (should stop at height-1)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello     \n
              \n
         X    \n
    ");
}

// Cursor Forward (C command) tests
#[test]
fn test_cursor_forward_basic() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[3D"); // Move back 3
    tty.stdout_write("\x1b[1C"); // Move forward 1
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    HelXo     \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_forward_multiple() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[5D"); // Move back 5
    tty.stdout_write("\x1b[2C"); // Move forward 2
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    HeXlo     \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_forward_no_parameter() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[3D"); // Move back 3
    tty.stdout_write("\x1b[C"); // Move forward 1 (default)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    HelXo     \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_forward_bounds_check() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[20C"); // Try to move forward 20 positions (should stop at width-1)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello    X\n
              \n
              \n
    ");
}

// Cursor Back (D command) tests
#[test]
fn test_cursor_back_basic() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[1D"); // Move back 1
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    HellX     \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_back_multiple() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[3D"); // Move back 3
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    HeXlo     \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_back_no_parameter() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[D"); // Move back 1 (default)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    HellX     \n
              \n
              \n
    ");
}

#[test]
fn test_cursor_back_bounds_check() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[20D"); // Try to move back 20 positions (should stop at 0)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Xello     \n
              \n
              \n
    ");
}

// =============================================================================
// ABSOLUTE CURSOR POSITIONING
// =============================================================================

// Cursor Position (H command) tests
#[test]
fn test_absolute_cursor_positioning() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello");
    tty.stdout_write("\x1b[1;1H"); // Move to top-left
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Xello     \n
              \n
              \n
    ");
}

#[test]
fn test_set_cursor_to_row_col() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Hello\nWorld");
    tty.stdout_write("\x1b[2;3H"); // Move to row 2, col 3
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello     \n
    WoXld     \n
              \n
              \n
              \n
    ");
}

#[test]
fn test_set_cursor_to_row_col_alt_syntax() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Hello\nWorld");
    tty.stdout_write("\x1b[2;3f"); // Move to row 2, col 3 (f command)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello     \n
    WoXld     \n
              \n
              \n
              \n
    ");
}

#[test]
fn test_set_cursor_to_home_position() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Hello\nWorld");
    tty.stdout_write("\x1b[H"); // Move to row 1, col 1 (default)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Xello     \n
    World     \n
              \n
              \n
              \n
    ");
}

#[test]
fn test_set_cursor_partial_coordinates() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Hello\nWorld");
    tty.stdout_write("\x1b[2;H"); // Move to row 2, col 1 (default)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello     \n
    Xorld     \n
              \n
              \n
              \n
    ");
}

#[test]
fn test_set_cursor_bounds_clamping() {
    let mut tty = VirtualTty::new(15, 10);
    tty.stdout_write("Hello\nWorld");
    tty.stdout_write("\x1b[20;30H"); // Try to move to row 20, col 30 (should be clamped)
    tty.stdout_write("X");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    World          \n
                   \n
                   \n
                   \n
                   \n
                   \n
                   \n
                   \n
                  X\n
                   \n
    ");
}
