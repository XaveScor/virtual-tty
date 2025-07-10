use std::io::Write;
use virtual_tty::VirtualTty;

// =============================================================================
// RELATIVE CURSOR MOVEMENTS
// =============================================================================

// Cursor Up (A command) tests
#[test]
fn test_cursor_up_basic() {
    let mut tty = VirtualTty::new(10, 5);
    write!(tty.stdout, "Line1\nLine2\nLine3").unwrap();
    write!(tty.stdout, "\x1b[1A").unwrap(); // Move up 1 line
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Line1\nLine2\nLine3").unwrap();
    write!(tty.stdout, "\x1b[2A").unwrap(); // Move up 2 lines
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Line1\nLine2").unwrap();
    write!(tty.stdout, "\x1b[A").unwrap(); // Move up 1 line (default)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[10A").unwrap(); // Try to move up 10 lines (should stop at 0)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Line1").unwrap();
    write!(tty.stdout, "\x1b[1B").unwrap(); // Move down 1 line
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Line1").unwrap();
    write!(tty.stdout, "\x1b[2B").unwrap(); // Move down 2 lines
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Line1").unwrap();
    write!(tty.stdout, "\x1b[B").unwrap(); // Move down 1 line (default)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[10B").unwrap(); // Try to move down 10 lines (should stop at height-1)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[3D").unwrap(); // Move back 3
    write!(tty.stdout, "\x1b[1C").unwrap(); // Move forward 1
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[5D").unwrap(); // Move back 5
    write!(tty.stdout, "\x1b[2C").unwrap(); // Move forward 2
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[3D").unwrap(); // Move back 3
    write!(tty.stdout, "\x1b[C").unwrap(); // Move forward 1 (default)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[20C").unwrap(); // Try to move forward 20 positions (should stop at width-1)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[1D").unwrap(); // Move back 1
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[3D").unwrap(); // Move back 3
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[D").unwrap(); // Move back 1 (default)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[20D").unwrap(); // Try to move back 20 positions (should stop at 0)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello").unwrap();
    write!(tty.stdout, "\x1b[1;1H").unwrap(); // Move to top-left
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello\nWorld").unwrap();
    write!(tty.stdout, "\x1b[2;3H").unwrap(); // Move to row 2, col 3
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello\nWorld").unwrap();
    write!(tty.stdout, "\x1b[2;3f").unwrap(); // Move to row 2, col 3 (f command)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello\nWorld").unwrap();
    write!(tty.stdout, "\x1b[H").unwrap(); // Move to row 1, col 1 (default)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello\nWorld").unwrap();
    write!(tty.stdout, "\x1b[2;H").unwrap(); // Move to row 2, col 1 (default)
    write!(tty.stdout, "X").unwrap();
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
    write!(tty.stdout, "Hello\nWorld").unwrap();
    write!(tty.stdout, "\x1b[20;30H").unwrap(); // Try to move to row 20, col 30 (should be clamped)
    write!(tty.stdout, "X").unwrap();
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
