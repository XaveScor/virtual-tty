use std::io::Write;
use virtual_tty::VirtualTty;

// =============================================================================
// MULTI-COMMAND SEQUENCES AND ADVANCED INTERACTIONS
// =============================================================================

#[test]
fn test_complex_cursor_sequence() {
    let mut tty = VirtualTty::new(10, 5);
    write!(tty.stdout, "Hello\nWorld\nTest").unwrap();
    write!(tty.stdout, "\x1b[2A").unwrap(); // Up 2 lines
    write!(tty.stdout, "\x1b[2C").unwrap(); // Right 2 columns
    write!(tty.stdout, "X").unwrap();
    write!(tty.stdout, "\x1b[1B").unwrap(); // Down 1 line
    write!(tty.stdout, "\x1b[1D").unwrap(); // Left 1 column
    write!(tty.stdout, "Y").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello X   \n
    World Y   \n
    Test      \n
              \n
              \n
    ");
}
