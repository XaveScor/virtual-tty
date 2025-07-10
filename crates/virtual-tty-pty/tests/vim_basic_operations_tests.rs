use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use virtual_tty_pty::PtyAdapter;

fn create_temp_file_from_fixture(fixture_path: &str, test_name: &str) -> String {
    let temp_file = format!("{}_{}.txt", test_name, std::process::id());
    std::fs::copy(fixture_path, &temp_file).unwrap();
    temp_file
}

#[test]
fn test_vim_startup_pty_state() {
    let temp_file = create_temp_file_from_fixture("tests/fixtures/simple_lines.txt", "startup");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(Command::new("vim").arg(&temp_file))
        .unwrap();

    // Wait for vim to fully initialize
    sleep(Duration::from_millis(1000));

    // PTY should capture vim's screen initialization
    let snapshot = pty.get_snapshot();
    // Normalize the filename to handle dynamic process IDs
    let normalized = snapshot.replace(
        &format!("startup_{}.txt", std::process::id()),
        "startup_PID.txt",
    );
    insta::assert_snapshot!(normalized, @r#"
    Line 1                                  \n
    Line 2                                  \n
    Line 3                                  \n
    Line 4                                  \n
    Line 5                                  \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "startup_PID.txt" [noeol] 5L, 34B      \n
    "#);

    // PTY cursor should be tracked correctly
    let (row, col) = pty.get_cursor_position();
    assert_eq!(row, 0, "PTY cursor should be at first line");
    assert_eq!(col, 0, "PTY cursor should be at first column");

    // Clean exit
    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();

    // Cleanup
    std::fs::remove_file(&temp_file).ok();
}

#[test]
fn test_vim_navigation_pty_cursor() {
    let temp_file = create_temp_file_from_fixture("tests/fixtures/simple_lines.txt", "navigation");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(Command::new("vim").arg(&temp_file))
        .unwrap();
    sleep(Duration::from_millis(500));

    // Test PTY cursor tracking with vim navigation
    let test_moves = [
        ("j", (1, 0)),  // Down
        ("j", (2, 0)),  // Down
        ("k", (1, 0)),  // Up
        ("$", (1, 5)),  // End of line (Line 2 = 6 chars, 0-indexed = 5)
        ("0", (1, 0)),  // Beginning of line
        ("w", (1, 5)),  // Word forward (to end of "Line 2")
        ("G", (4, 0)),  // Go to last line
        ("gg", (0, 0)), // Go to first line
    ];

    for (key, expected_pos) in test_moves {
        pty.send_input_str(key).unwrap();
        sleep(Duration::from_millis(100));

        let (row, col) = pty.get_cursor_position();
        assert_eq!(
            (row, col),
            expected_pos,
            "PTY cursor tracking failed for key '{}', expected {:?}, got {:?}",
            key,
            expected_pos,
            (row, col)
        );
    }

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();

    // Cleanup
    std::fs::remove_file(&temp_file).ok();
}

#[test]
fn test_vim_insert_mode_pty_buffer() {
    let temp_file = create_temp_file_from_fixture("tests/fixtures/single_line.txt", "insert");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(Command::new("vim").arg(&temp_file))
        .unwrap();
    sleep(Duration::from_millis(500));

    // Enter insert mode
    pty.send_input_str("i").unwrap();
    sleep(Duration::from_millis(100));

    // PTY should capture mode indicator
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Single line of text for testing         \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- INSERT --                            \n
    ");

    // Type text and validate PTY buffer updates
    let test_text = "Hello PTY ";
    pty.send_input_str(test_text).unwrap();
    sleep(Duration::from_millis(200));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello PTY Single line of text for testin\n
    g                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- INSERT --                            \n
    ");

    // Exit insert mode
    pty.send_input_str("\x1b").unwrap(); // ESC
    sleep(Duration::from_millis(100));

    // PTY should no longer show insert mode
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Hello PTY Single line of text for testin\n
    g                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
                                            \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();

    // Cleanup
    std::fs::remove_file(&temp_file).ok();
}

#[test]
fn test_vim_delete_operations_pty() {
    let temp_file = create_temp_file_from_fixture("tests/fixtures/basic_content.txt", "delete");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(Command::new("vim").arg(&temp_file))
        .unwrap();
    sleep(Duration::from_millis(500));

    // Test character deletion
    pty.send_input_str("x").unwrap(); // Delete first character
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    // Normalize the filename to handle dynamic process IDs
    let normalized = snapshot.replace(
        &format!("delete_{}.txt", std::process::id()),
        "delete_PID.txt",
    );
    insta::assert_snapshot!(normalized, @r#"
    ine 1                                   \n
    Line 2                                  \n
    Line 3                                  \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "delete_PID.txt" [noeol] 3L, 22B       \n
    "#);

    // Test line deletion (using D command which works reliably)
    pty.send_input_str("D").unwrap(); // Delete to end of line
    sleep(Duration::from_millis(300));

    let snapshot = pty.get_snapshot();
    // Normalize the filename to handle dynamic process IDs
    let normalized_d = snapshot.replace(
        &format!("delete_{}.txt", std::process::id()),
        "delete_PID.txt",
    );
    insta::assert_snapshot!(normalized_d, @r#"
                                            \n
    Line 2                                  \n
    Line 3                                  \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "delete_PID.txt" [noeol] 3L, 22B       \n
    "#);

    // Test word deletion - move to Line 2 first and then delete word
    pty.send_input_str("j").unwrap(); // Move to Line 2
    sleep(Duration::from_millis(100));
    pty.send_input_str("dw").unwrap(); // Delete word "Line"
    sleep(Duration::from_millis(200));

    let snapshot = pty.get_snapshot();
    // Normalize the filename to handle dynamic process IDs
    let normalized_dw = snapshot.replace(
        &format!("delete_{}.txt", std::process::id()),
        "delete_PID.txt",
    );
    insta::assert_snapshot!(normalized_dw, @r#"
                                            \n
    2                                       \n
    Line 3                                  \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "delete_PID.txt" [noeol] 3L, 22B       \n
    "#);

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();

    // Cleanup
    std::fs::remove_file(&temp_file).ok();
}

#[test]
fn test_vim_undo_redo_pty_state() {
    let temp_file = create_temp_file_from_fixture("tests/fixtures/single_line.txt", "undo");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(Command::new("vim").arg(&temp_file))
        .unwrap();
    sleep(Duration::from_millis(500));

    // Make changes
    pty.send_input_str("A Modified").unwrap(); // Append text
    pty.send_input_str("\x1b").unwrap(); // ESC
    sleep(Duration::from_millis(100));

    let modified_snapshot = pty.get_snapshot();
    insta::assert_snapshot!(modified_snapshot, @r"
    Single line of text for testing Modified\n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- INSERT --                            \n
    ");

    // Test undo
    pty.send_input_str("u").unwrap();
    sleep(Duration::from_millis(100));

    let undo_snapshot = pty.get_snapshot();
    // Normalize the timing to handle variable timing
    let normalized_undo = undo_snapshot
        .replace("1 second ago", "X seconds ago")
        .replace("0 seconds ago", "X seconds ago");
    insta::assert_snapshot!(normalized_undo, @r"
    Single line of text for testing         \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    1 change; before #1  X seconds ago      \n
    ");

    // Test redo
    pty.send_input_str("\x12").unwrap(); // Ctrl+R
    sleep(Duration::from_millis(100));

    let redo_snapshot = pty.get_snapshot();
    // Normalize the timing to handle variable timing
    let normalized_redo = redo_snapshot
        .replace("1 second ago", "X seconds ago")
        .replace("0 seconds ago", "X seconds ago");
    insta::assert_snapshot!(normalized_redo, @r"
    Single line of text for testing Modified\n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    1 change; after #1  X seconds ago        \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();

    // Cleanup
    std::fs::remove_file(&temp_file).ok();
}

#[test]
fn test_vim_command_mode_pty() {
    let temp_file = create_temp_file_from_fixture("tests/fixtures/basic_content.txt", "command");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(Command::new("vim").arg(&temp_file))
        .unwrap();
    sleep(Duration::from_millis(500));

    // Enter command mode
    pty.send_input_str(":").unwrap();
    sleep(Duration::from_millis(100));

    // PTY should show command prompt
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line 1                                  \n
    Line 2                                  \n
    Line 3                                  \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    :                                       \n
    ");

    // Type command
    pty.send_input_str("set number").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line 1                                  \n
    Line 2                                  \n
    Line 3                                  \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    :set number                             \n
    ");

    // Execute command
    pty.send_input_str("\n").unwrap();
    sleep(Duration::from_millis(200));

    // PTY should show line numbers
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
      1 Line 1                              \n
      2 Line 2                              \n
      3 Line 3                              \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    :set number                             \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();

    // Cleanup
    std::fs::remove_file(&temp_file).ok();
}
