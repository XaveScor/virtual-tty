use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use virtual_tty_pty::PtyAdapter;

fn wait_for_output() {
    sleep(Duration::from_millis(50));
}

#[test]
fn test_cursor_up_preserves_output() {
    let mut pty = PtyAdapter::new(80, 24);

    // Write initial content
    let mut child = pty
        .spawn_command(Command::new("printf").arg("Line1\nLine2\nLine3"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let baseline_snapshot = pty.get_snapshot();
    let (initial_row, initial_col) = pty.get_cursor_position();

    // Send cursor up command
    let mut child = pty
        .spawn_command(Command::new("printf").arg("\x1b[A"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let after_cursor_snapshot = pty.get_snapshot();
    let (after_row, after_col) = pty.get_cursor_position();

    // Terminal content should be identical
    assert_eq!(
        baseline_snapshot, after_cursor_snapshot,
        "Cursor up should not change terminal content"
    );

    // But cursor position should change
    assert_eq!(after_row, initial_row - 1, "Cursor should move up one row");
    assert_eq!(after_col, initial_col, "Cursor column should not change");

    // Validate the actual content with snapshot
    insta::assert_snapshot!(baseline_snapshot, @r"
Line1                                                                           \n
Line2                                                                           \n
Line3                                                                           \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
                                                                                \n
");

    pty.wait_for_completion();
}

#[test]
fn test_cursor_down_preserves_output() {
    let mut pty = PtyAdapter::new(80, 24);

    // Write initial content and position cursor at start
    let mut child = pty
        .spawn_command(Command::new("printf").arg("Line1\nLine2\x1b[1;1H"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let baseline_snapshot = pty.get_snapshot();
    let (initial_row, initial_col) = pty.get_cursor_position();

    // Send cursor down command
    let mut child = pty
        .spawn_command(Command::new("printf").arg("\x1b[B"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let after_cursor_snapshot = pty.get_snapshot();
    let (after_row, after_col) = pty.get_cursor_position();

    // Terminal content should be identical
    assert_eq!(
        baseline_snapshot, after_cursor_snapshot,
        "Cursor down should not change terminal content"
    );

    // But cursor position should change
    assert_eq!(
        after_row,
        initial_row + 1,
        "Cursor should move down one row"
    );
    assert_eq!(after_col, initial_col, "Cursor column should not change");

    pty.wait_for_completion();
}

#[test]
fn test_cursor_forward_preserves_output() {
    let mut pty = PtyAdapter::new(80, 24);

    // Write initial content
    let mut child = pty
        .spawn_command(Command::new("printf").arg("Hello World"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let baseline_snapshot = pty.get_snapshot();
    let (initial_row, initial_col) = pty.get_cursor_position();

    // Send cursor forward command
    let mut child = pty
        .spawn_command(Command::new("printf").arg("\x1b[3C"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let after_cursor_snapshot = pty.get_snapshot();
    let (after_row, after_col) = pty.get_cursor_position();

    // Terminal content should be identical
    assert_eq!(
        baseline_snapshot, after_cursor_snapshot,
        "Cursor forward should not change terminal content"
    );

    // But cursor position should change
    assert_eq!(after_row, initial_row, "Cursor row should not change");
    assert_eq!(
        after_col,
        initial_col + 3,
        "Cursor should move forward 3 columns"
    );

    pty.wait_for_completion();
}

#[test]
fn test_cursor_back_preserves_output() {
    let mut pty = PtyAdapter::new(80, 24);

    // Write initial content
    let mut child = pty
        .spawn_command(Command::new("printf").arg("Hello World"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let baseline_snapshot = pty.get_snapshot();
    let (initial_row, initial_col) = pty.get_cursor_position();

    // Send cursor back command
    let mut child = pty
        .spawn_command(Command::new("printf").arg("\x1b[5D"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let after_cursor_snapshot = pty.get_snapshot();
    let (after_row, after_col) = pty.get_cursor_position();

    // Terminal content should be identical
    assert_eq!(
        baseline_snapshot, after_cursor_snapshot,
        "Cursor back should not change terminal content"
    );

    // But cursor position should change
    assert_eq!(after_row, initial_row, "Cursor row should not change");
    assert_eq!(
        after_col,
        initial_col - 5,
        "Cursor should move back 5 columns"
    );

    pty.wait_for_completion();
}

#[test]
fn test_absolute_cursor_positioning_preserves_output() {
    let mut pty = PtyAdapter::new(80, 24);

    // Write initial content
    let mut child = pty
        .spawn_command(Command::new("printf").arg("Line1\nLine2\nLine3"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let baseline_snapshot = pty.get_snapshot();

    // Send absolute cursor positioning command
    let mut child = pty
        .spawn_command(Command::new("printf").arg("\x1b[2;5H"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let after_cursor_snapshot = pty.get_snapshot();
    let (after_row, after_col) = pty.get_cursor_position();

    // Terminal content should be identical
    assert_eq!(
        baseline_snapshot, after_cursor_snapshot,
        "Absolute cursor positioning should not change terminal content"
    );

    // But cursor position should be at specified location
    assert_eq!(
        after_row, 1,
        "Cursor should be at row 1 (0-indexed from row 2)"
    );
    assert_eq!(
        after_col, 4,
        "Cursor should be at column 4 (0-indexed from column 5)"
    );

    pty.wait_for_completion();
}

#[test]
fn test_multiple_cursor_commands_preserve_output() {
    let mut pty = PtyAdapter::new(80, 24);

    // Write initial content across multiple lines
    let mut child = pty
        .spawn_command(Command::new("printf").arg("First\nSecond\nThird\nFourth"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let baseline_snapshot = pty.get_snapshot();

    // Send sequence of cursor commands
    let mut child = pty
        .spawn_command(Command::new("printf").arg("\x1b[A\x1b[A\x1b[3C\x1b[2D\x1b[B\x1b[4;2H"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let after_cursor_snapshot = pty.get_snapshot();
    let (final_row, final_col) = pty.get_cursor_position();

    // Terminal content should be identical after all cursor movements
    assert_eq!(
        baseline_snapshot, after_cursor_snapshot,
        "Multiple cursor commands should not change terminal content"
    );

    // Final cursor position should be from last absolute positioning command (ESC[4;2H)
    assert_eq!(
        final_row, 3,
        "Final cursor should be at row 3 (0-indexed from row 4)"
    );
    assert_eq!(
        final_col, 1,
        "Final cursor should be at column 1 (0-indexed from column 2)"
    );

    pty.wait_for_completion();
}

#[test]
fn test_cursor_boundary_commands_preserve_output() {
    let mut pty = PtyAdapter::new(80, 24);

    // Write content and position cursor at top-left
    let mut child = pty
        .spawn_command(Command::new("printf").arg("Test\x1b[1;1H"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let baseline_snapshot = pty.get_snapshot();

    // Try to move cursor beyond boundaries
    let mut child = pty
        .spawn_command(Command::new("printf").arg("\x1b[A\x1b[D")) // Up from row 0, left from col 0
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let after_cursor_snapshot = pty.get_snapshot();
    let (after_row, after_col) = pty.get_cursor_position();

    // Terminal content should be identical
    assert_eq!(
        baseline_snapshot, after_cursor_snapshot,
        "Boundary cursor commands should not change terminal content"
    );

    // Cursor should stay at boundaries
    assert_eq!(
        after_row, 0,
        "Cursor should stay at row 0 when trying to move up from top"
    );
    assert_eq!(
        after_col, 0,
        "Cursor should stay at column 0 when trying to move left from leftmost"
    );

    pty.wait_for_completion();
}

#[test]
fn test_cursor_positioning_with_content_validation() {
    let mut pty = PtyAdapter::new(10, 5); // Smaller terminal for focused test

    // Write content to specific positions to test positioning accuracy
    let mut child = pty
        .spawn_command(Command::new("printf").arg("\x1b[1;1HA\x1b[2;2HB\x1b[3;3HC"))
        .expect("Failed to spawn printf");
    child.wait().unwrap();
    wait_for_output();

    let (final_row, final_col) = pty.get_cursor_position();
    let snapshot = pty.get_snapshot();

    // Validate exact cursor position
    assert_eq!(final_row, 2, "Final cursor should be at row 2");
    assert_eq!(
        final_col, 3,
        "Final cursor should be at column 3 (after writing 'C')"
    );

    // Validate terminal content with inline snapshot
    insta::assert_snapshot!(snapshot, @r"
A         \n
 B        \n
  C       \n
          \n
          \n
");

    pty.wait_for_completion();
}
