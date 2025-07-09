use virtual_tty::VirtualTty;

// =============================================================================
// MULTI-COMMAND SEQUENCES AND ADVANCED INTERACTIONS - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_complex_cursor_sequence() {
    let mut tty = VirtualTty::new(10, 5);
    tty.stdout_write("Hello\nWorld\nTest");
    tty.stdout_write("\x1b[2A"); // Up 2 lines
    tty.stderr_write("\x1b[2C"); // Right 2 columns
    tty.stdout_write("X");
    tty.stderr_write("\x1b[1B"); // Down 1 line
    tty.stdout_write("\x1b[1D"); // Left 1 column
    tty.stderr_write("Y");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Hello X\nWorld Y\nTest");
}

#[test]
fn test_mixed_interleaved_output() {
    let mut tty = VirtualTty::new(35, 4);
    tty.stdout_write("Command: ");
    tty.stderr_write("ERROR: ");
    tty.stdout_write("ls -la");
    tty.stderr_write("Permission denied");
    tty.stdout_write("\n");
    tty.stderr_write("\n");
    tty.stdout_write("Exit code: 1");
    let snapshot = tty.get_snapshot();
    assert_eq!(
        snapshot,
        "Command: ERROR: ls -laPermission de\nnied\n\nExit code: 1"
    );
}

#[test]
fn test_mixed_ansi_sequences() {
    let mut tty = VirtualTty::new(15, 3);
    tty.stdout_write("Normal text");
    tty.stderr_write("\x1b[H"); // Home position
    tty.stdout_write("Override");
    tty.stderr_write("\x1b[2J"); // Clear screen
    tty.stdout_write("Cleared");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Cleared");
}

#[test]
fn test_mixed_multiline_output() {
    let mut tty = VirtualTty::new(12, 4);
    tty.stdout_write("Line 1\n");
    tty.stderr_write("Error 1\n");
    tty.stdout_write("Line 2\n");
    tty.stderr_write("Error 2");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Line 1\nError 1\nLine 2\nError 2");
}

#[test]
fn test_mixed_cursor_positioning() {
    let mut tty = VirtualTty::new(10, 3);
    tty.stdout_write("Start");
    tty.stderr_write("\x1b[1;3H"); // Position 1,3
    tty.stdout_write("Mid");
    tty.stderr_write("\x1b[2;1H"); // Position 2,1
    tty.stdout_write("End");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "StMid\nEnd");
}

#[test]
fn test_mixed_line_wrapping() {
    let mut tty = VirtualTty::new(8, 3);
    tty.stdout_write("Long");
    tty.stderr_write("Line");
    tty.stdout_write("That");
    tty.stderr_write("Wraps");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "LongLine\nThatWrap\ns");
}

#[test]
fn test_mixed_scrolling_behavior() {
    let mut tty = VirtualTty::new(10, 2);
    tty.stdout_write("Line1\n");
    tty.stderr_write("Line2\n");
    tty.stdout_write("Line3\n");
    tty.stderr_write("Line4");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Line3\nLine4");
}

#[test]
fn test_mixed_empty_writes() {
    let mut tty = VirtualTty::new(10, 2);
    tty.stdout_write("Start");
    tty.stderr_write("");
    tty.stdout_write(" ");
    tty.stderr_write("");
    tty.stdout_write("End");
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "Start End");
}

#[test]
fn test_mixed_rapid_alternation() {
    let mut tty = VirtualTty::new(25, 3);
    for i in 0..5 {
        tty.stdout_write(&format!("O{}", i));
        tty.stderr_write(&format!("E{}", i));
    }
    let snapshot = tty.get_snapshot();
    assert_eq!(snapshot, "O0E0O1E1O2E2O3E3O4E4");
}

#[test]
fn test_mixed_command_error_pattern() {
    let mut tty = VirtualTty::new(25, 4);
    tty.stdout_write("$ command --option\n");
    tty.stderr_write("ERROR: Invalid option\n");
    tty.stdout_write("Usage: command [args]\n");
    tty.stderr_write("Exit: 1");
    let snapshot = tty.get_snapshot();
    assert_eq!(
        snapshot,
        "$ command --option\nERROR: Invalid option\nUsage: command [args]\nExit: 1"
    );
}
