use std::io::Write;
use virtual_tty::VirtualTty;

// =============================================================================
// MULTI-COMMAND SEQUENCES AND ADVANCED INTERACTIONS - MIXED STDOUT/STDERR
// =============================================================================

#[test]
fn test_mixed_complex_cursor_sequence() {
    let mut tty = VirtualTty::new(10, 5);
    write!(tty.stdout, "Hello\nWorld\nTest").unwrap();
    write!(tty.stdout, "\x1b[2A").unwrap(); // Up 2 lines
    write!(tty.stderr, "\x1b[2C").unwrap(); // Right 2 columns
    write!(tty.stdout, "X").unwrap();
    write!(tty.stderr, "\x1b[1B").unwrap(); // Down 1 line
    write!(tty.stdout, "\x1b[1D").unwrap(); // Left 1 column
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

#[test]
fn test_mixed_interleaved_output() {
    let mut tty = VirtualTty::new(35, 4);
    write!(tty.stdout, "Command: ").unwrap();
    write!(tty.stderr, "ERROR: ").unwrap();
    write!(tty.stdout, "ls -la").unwrap();
    write!(tty.stderr, "Permission denied").unwrap();
    write!(tty.stdout, "\n").unwrap();
    write!(tty.stderr, "\n").unwrap();
    write!(tty.stdout, "Exit code: 1").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Command: ERROR: ls -laPermission de\n
    nied                               \n
                                       \n
    Exit code: 1                       \n
    ");
}

#[test]
fn test_mixed_ansi_sequences() {
    let mut tty = VirtualTty::new(15, 3);
    write!(tty.stdout, "\x1b[H").unwrap(); // Home position
    write!(tty.stderr, "\x1b[2J").unwrap(); // Clear screen
    write!(tty.stdout, "Cleared").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Cleared        \n
                   \n
                   \n
    ");
}

#[test]
fn test_mixed_multiline_output() {
    let mut tty = VirtualTty::new(12, 4);
    write!(tty.stdout, "Line 1\n").unwrap();
    write!(tty.stderr, "Error 1\n").unwrap();
    write!(tty.stdout, "Line 2\n").unwrap();
    write!(tty.stderr, "Error 2").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line 1      \n
    Error 1     \n
    Line 2      \n
    Error 2     \n
    ");
}

#[test]
fn test_mixed_cursor_positioning() {
    let mut tty = VirtualTty::new(10, 3);
    write!(tty.stdout, "Start").unwrap();
    write!(tty.stderr, "\x1b[1;3H").unwrap(); // Position 1,3
    write!(tty.stdout, "Mid").unwrap();
    write!(tty.stderr, "\x1b[2;1H").unwrap(); // Position 2,1
    write!(tty.stdout, "End").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    StMid     \n
    End       \n
              \n
    ");
}

#[test]
fn test_mixed_line_wrapping() {
    let mut tty = VirtualTty::new(8, 3);
    write!(tty.stdout, "LongLine").unwrap();
    write!(tty.stderr, "ThatWrap").unwrap();
    write!(tty.stdout, "s").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    LongLine\n
    ThatWrap\n
    s       \n
    ");
}

#[test]
fn test_mixed_scrolling_behavior() {
    let mut tty = VirtualTty::new(10, 2);
    write!(tty.stdout, "Line1\n").unwrap();
    write!(tty.stderr, "Line2\n").unwrap();
    write!(tty.stdout, "Line3\n").unwrap();
    write!(tty.stderr, "Line4").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line3     \n
    Line4     \n
    ");
}

#[test]
fn test_mixed_empty_writes() {
    let mut tty = VirtualTty::new(10, 2);
    write!(tty.stdout, "Start").unwrap();
    write!(tty.stderr, "").unwrap();
    write!(tty.stdout, " ").unwrap();
    write!(tty.stderr, "").unwrap();
    write!(tty.stdout, "End").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Start End \n
              \n
    ");
}

#[test]
fn test_mixed_rapid_alternation() {
    let mut tty = VirtualTty::new(25, 3);
    for i in 0..5 {
        write!(tty.stdout, "O{}", i).unwrap();
        write!(tty.stderr, "E{}", i).unwrap();
    }
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    O0E0O1E1O2E2O3E3O4E4     \n
                             \n
                             \n
    ");
}

#[test]
fn test_mixed_command_error_pattern() {
    let mut tty = VirtualTty::new(25, 4);
    write!(tty.stdout, "$ command --option\n").unwrap();
    write!(tty.stderr, "ERROR: Invalid option\n").unwrap();
    write!(tty.stdout, "Usage: command [args]\n").unwrap();
    write!(tty.stderr, "Exit: 1").unwrap();
    let snapshot = tty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    $ command --option       \n
    ERROR: Invalid option    \n
    Usage: command [args]    \n
    Exit: 1                  \n
    ");
}
