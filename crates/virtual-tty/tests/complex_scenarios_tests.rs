use virtual_tty::VirtualTty;

// =============================================================================
// MULTI-COMMAND SEQUENCES AND ADVANCED INTERACTIONS
// =============================================================================

#[test]
fn test_complex_cursor_sequence() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Hello\nWorld\nTest");
    tty.stdout_write("\x1b[2A"); // Up 2 lines
    tty.stdout_write("\x1b[2C"); // Right 2 columns
    tty.stdout_write("X");
    tty.stdout_write("\x1b[1B"); // Down 1 line
    tty.stdout_write("\x1b[1D"); // Left 1 column
    tty.stdout_write("Y");
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello X   \n
    World Y   \n
    Test      \n
              \n
              \n
    ");
}
