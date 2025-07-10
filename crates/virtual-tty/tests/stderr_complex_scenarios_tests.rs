use std::io::Write;
use virtual_tty::VirtualTty;

// =============================================================================
// MULTI-COMMAND SEQUENCES AND ADVANCED INTERACTIONS
// =============================================================================

#[test]
fn test_stderr_complex_cursor_sequence() {
    let mut tty = VirtualTty::new(10, 5);
    write!(tty.stderr, "Hello\nWorld\nTest").unwrap();
    write!(tty.stderr, "\x1b[2A").unwrap(); // Up 2 lines
    write!(tty.stderr, "\x1b[2C").unwrap(); // Right 2 columns
    write!(tty.stderr, "X").unwrap();
    write!(tty.stderr, "\x1b[1B").unwrap(); // Down 1 line
    write!(tty.stderr, "\x1b[1D").unwrap(); // Left 1 column
    write!(tty.stderr, "Y").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello X   \n
    World Y   \n
    Test      \n
              \n
              \n
    ");
}
