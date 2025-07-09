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
    assert_eq!(snapshot, "He123");
}

#[test]
fn test_clear_entire_line() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Hello\nWorld\nTest");
    tty.stdout_write("\x1b[2A"); // Move up 2 lines
    tty.stdout_write("\x1b[2K"); // Clear entire line
    tty.stdout_write("New");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "    New\nWorld\nTest");
}
