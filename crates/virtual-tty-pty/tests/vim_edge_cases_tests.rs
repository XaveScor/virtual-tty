use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use tempfile::TempDir;
use virtual_tty_pty::PtyAdapter;
#[allow(dead_code)]
fn copy_fixture_to_dir(dir: &Path, fixture_name: &str, target_name: &str) {
    let fixture_path = Path::new("tests/fixtures").join(fixture_name);
    let target_path = dir.join(target_name);
    fs::copy(&fixture_path, &target_path).unwrap();
}

// Terminal Size Edge Cases
#[test]
fn test_vim_tiny_terminal_pty() {
    let temp_dir = TempDir::new().unwrap();

    let content = "This is a very long line that will definitely exceed the tiny terminal width";
    let test_file = temp_dir.path().join("tiny_test.txt");
    fs::write(&test_file, content).unwrap();

    let mut pty = PtyAdapter::new(10, 5);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("tiny_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(1000));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    This is a \n
    ry long li\n
     that will\n
    efinitely \n
    ex        \n
    ");

    let (row, col) = pty.get_cursor_position();
    assert!(row < 5, "PTY cursor row should be within terminal bounds");
    assert!(
        col < 10,
        "PTY cursor column should be within terminal bounds"
    );

    pty.send_input_str("$").unwrap();
    sleep(Duration::from_millis(100));

    let (_row, col) = pty.get_cursor_position();
    assert!(col < 10, "PTY cursor should stay within terminal width");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_huge_terminal_pty() {
    let temp_dir = TempDir::new().unwrap();

    let content = "Small content in huge terminal";
    let test_file = temp_dir.path().join("huge_test.txt");
    fs::write(&test_file, content).unwrap();

    let mut pty = PtyAdapter::new(200, 60);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("huge_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(1000));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    Small content in huge terminal                                                                                                                                                                          \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    ~                                                                                                                                                                                                       \n
    "huge_test.txt" [noeol] 1L, 30B                                                                                                                                                                         \n
    "#);

    let start_time = std::time::Instant::now();
    for _ in 0..10 {
        pty.send_input_str("G").unwrap();
        pty.send_input_str("gg").unwrap();
        sleep(Duration::from_millis(10));
    }
    let nav_time = start_time.elapsed();

    assert!(
        nav_time < Duration::from_millis(500),
        "PTY should handle navigation efficiently in large terminal"
    );

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_square_terminal_pty() {
    let temp_dir = TempDir::new().unwrap();

    let content = "Testing square terminal aspect ratio with various content lengths";
    let test_file = temp_dir.path().join("square_test.txt");
    fs::write(&test_file, content).unwrap();

    let mut pty = PtyAdapter::new(40, 40);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("square_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    Testing square terminal aspect ratio wit\n
    h various content lengths               \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "square_test.txt" [noeol] 1L, 65B       \n
    "#);

    let all_lines: Vec<&str> = snapshot.lines().collect();

    let first_non_empty = all_lines
        .iter()
        .position(|line| !line.trim().is_empty())
        .unwrap_or(0);
    let last_non_empty = all_lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .unwrap_or(all_lines.len().saturating_sub(1));

    let meaningful_lines = if first_non_empty <= last_non_empty {
        &all_lines[first_non_empty..=last_non_empty]
    } else {
        &[]
    };

    assert!(meaningful_lines.len() <= 40,
        "PTY content should not exceed terminal height (got {} meaningful lines from {} total lines)",
        meaningful_lines.len(), all_lines.len());

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

// File Content Edge Cases
#[test]
fn test_vim_empty_file_pty() {
    let temp_dir = TempDir::new().unwrap();

    let test_file = temp_dir.path().join("empty.txt");
    fs::write(&test_file, "").unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("empty.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
                                            \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "empty.txt" 0L, 0B                      \n
    "#);

    let (row, col) = pty.get_cursor_position();
    assert_eq!(row, 0, "PTY cursor should be at row 0 for empty file");
    assert_eq!(col, 0, "PTY cursor should be at column 0 for empty file");

    pty.send_input_str("i").unwrap();
    pty.send_input_str("First content").unwrap();
    pty.send_input_str("\x1b").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    First content                           \n
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

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_unicode_content_pty() {
    let temp_dir = TempDir::new().unwrap();

    let unicode_content = "Unicode test: 🚀 émojis and açcénts\n中文字符\nЯзык тест\n🎉🔥✨";
    let test_file = temp_dir.path().join("unicode.txt");
    fs::write(&test_file, unicode_content).unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("unicode.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    Unicode test: 🚀  émojis and açcénts     \n
    中文字符                                    \n
    Язык тест                               \n
    🎉 🔥 ✨                                   \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "unicode.txt" [noeol] 4L, 83B           \n
    "#);

    pty.send_input_str("j").unwrap();
    sleep(Duration::from_millis(100));

    let (row, _col) = pty.get_cursor_position();
    assert_eq!(row, 1, "PTY should navigate correctly with Unicode content");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_binary_file_pty() {
    let temp_dir = TempDir::new().unwrap();

    let binary_data = vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD];
    let test_file = temp_dir.path().join("binary.bin");
    fs::write(&test_file, binary_data).unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("binary.bin")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    ^@^A^B^Cÿþý                             \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "binary.bin" [noeol][converted] 1L, 10B \n
    "#);

    let (row, _col) = pty.get_cursor_position();
    assert!(
        row < 10,
        "PTY cursor should be within bounds for binary file"
    );

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_very_long_lines_pty() {
    let temp_dir = TempDir::new().unwrap();

    let long_line = "A".repeat(5000);
    let test_file = temp_dir.path().join("long_line.txt");
    fs::write(&test_file, long_line).unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("long_line.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(1000));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n
    "long_line.txt" [noeol] 1L, 5000B       \n
    "#);

    pty.send_input_str("$").unwrap();
    sleep(Duration::from_millis(200));

    let (row, _col) = pty.get_cursor_position();
    assert!(
        row > 0,
        "Cursor should be on a wrapped portion of the line when at end"
    );
    assert!(row < 10, "Cursor should be within terminal bounds");

    let start_time = std::time::Instant::now();
    pty.send_input_str("0").unwrap();
    sleep(Duration::from_millis(100));
    let nav_time = start_time.elapsed();

    let (row, _col) = pty.get_cursor_position();
    assert_eq!(
        row, 0,
        "Cursor should be at row 0 when at beginning of line"
    );

    assert!(
        nav_time < Duration::from_millis(500),
        "PTY should handle long line navigation efficiently"
    );

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

// Input Edge Cases
#[test]
fn test_vim_rapid_input_pty() {
    let temp_dir = TempDir::new().unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("rapid_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    pty.send_input_str("i").unwrap();

    let rapid_text = "abcdefghijklmnopqrstuvwxyz".repeat(10);
    pty.send_input_str(&rapid_text).unwrap();
    pty.send_input_str("\x1b").unwrap();
    sleep(Duration::from_millis(200));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    abcdefghijklmnopqrstuvwxyzabcdefghijklmn\n
    opqrstuvwxyzabcdefghijklmnopqrstuvwxyzab\n
    cdefghijklmnopqrstuvwxyzabcdefghijklmnop\n
    qrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd\n
    efghijklmnopqrstuvwxyzabcdefghijklmnopqr\n
    stuvwxyzabcdefghijklmnopqrstuvwxyzabcdef\n
    ghijklmnopqrstuvwxyz                    \n
    ~                                       \n
    ~                                       \n
    -- INSERT --                            \n
    ");

    for _ in 0..50 {
        pty.send_input_str("x").unwrap();
        sleep(Duration::from_millis(5));
    }

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    abcdefghijklmnopqrstuvwxyzabcdefghijklmn\n
    opqrstuvwxyzabcdefghijklmnopqrstuvwxyzab\n
    cdefghijklmnopqrstuvwxyzabcdefghijklmnop\n
    qrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd\n
    efghijklmnopqrstuvwxyzabcdefghijklmnopqr\n
    stuvwxyzab                              \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
                                            \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_control_character_input_pty() {
    let temp_dir = TempDir::new().unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("ctrl_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    pty.send_input_str("i").unwrap();

    pty.send_input_str("Before\tAfter").unwrap();
    pty.send_input_str("\n").unwrap();

    pty.send_input_str("Delete\x08\x08\x08").unwrap();

    pty.send_input_str("\rOverwrite").unwrap();

    pty.send_input_str("\x1b").unwrap();
    sleep(Duration::from_millis(200));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Before  After                           \n
    Del                                     \n
    Overwrite                               \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- INSERT --                            \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_escape_sequence_input_pty() {
    let temp_dir = TempDir::new().unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("escape_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    pty.send_input_str("i").unwrap();

    pty.send_input_str("Text with \x1b[31mcolor\x1b[0m codes")
        .unwrap();
    pty.send_input_str("\x1b").unwrap();
    sleep(Duration::from_millis(200));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Text with                               \n
    l                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- INSERT --                            \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

// Error Conditions and Recovery
#[test]
fn test_vim_nonexistent_file_pty() {
    let temp_dir = TempDir::new().unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("nonexistent.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
                                            \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "nonexistent.txt" [New]                 \n
    "#);

    pty.send_input_str("i").unwrap();
    pty.send_input_str("New file content").unwrap();
    pty.send_input_str("\x1b").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    New file content                        \n
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

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_permission_error_pty() {
    let temp_dir = TempDir::new().unwrap();

    let test_file = temp_dir.path().join("readonly.txt");
    fs::write(&test_file, "Read-only content").unwrap();
    let mut perms = fs::metadata(&test_file).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&test_file, perms).unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("readonly.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    pty.send_input_str("i").unwrap();
    pty.send_input_str("Modified").unwrap();
    pty.send_input_str("\x1b").unwrap();
    sleep(Duration::from_millis(100));

    pty.send_input_str(":w\n").unwrap();
    sleep(Duration::from_millis(200));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- INSERT -- W10: Warning: Changing a re\n
    adonly file                             \n
    Press ENTER or type command to continue \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_out_of_memory_simulation_pty() {
    let temp_dir = TempDir::new().unwrap();

    let large_content = "Large line content ".repeat(10000);
    let test_file = temp_dir.path().join("memory_test.txt");
    fs::write(&test_file, large_content).unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("memory_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(2000));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    Large line content Large line content La\n
    rge line content Large line content Larg\n
    e line content Large line content Large \n
    line content Large line content Large li\n
    ne content Large line content Large line\n
     content Large line content Large line c\n
    ontent Large line content Large line con\n
    tent Large line content Large line conte\n
    nt Large line content Large line content\n
    "memory_test.txt" [noeol] 1L, 190000B   \n
    "#);

    pty.send_input_str("G").unwrap();
    sleep(Duration::from_millis(200));

    let (row, _col) = pty.get_cursor_position();
    assert!(row < 10, "PTY cursor should be within bounds");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

// Signal Handling and Interruption
#[test]
fn test_vim_interrupt_handling_pty() {
    let temp_dir = TempDir::new().unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("interrupt_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    pty.send_input_str(":set number\n").unwrap();
    pty.send_input_str("i").unwrap();

    pty.send_input_str("Some content").unwrap();

    pty.send_input_str("\x03").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
                                            \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    Type  :qa  and pre...Enter> to exit Vim \n
    ");

    pty.send_input_str("\x1b").unwrap();
    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_sudden_termination_pty() {
    let temp_dir = TempDir::new().unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("term_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    pty.send_input_str("i").unwrap();
    pty.send_input_str("Content before termination").unwrap();
    pty.send_input_str("\x1b").unwrap();
    sleep(Duration::from_millis(100));

    child.kill().unwrap();

    pty.wait_for_completion();

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Content before termination              \n
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
}

// Resource Exhaustion Edge Cases
#[test]
fn test_vim_many_files_pty() {
    let temp_dir = TempDir::new().unwrap();

    for i in 0..20 {
        let test_file = temp_dir.path().join(format!("file_{i}.txt"));
        fs::write(&test_file, format!("Content {i}")).unwrap();
    }

    let mut pty = PtyAdapter::new(40, 10);
    let mut cmd = Command::new("vim");
    for i in 0..20 {
        cmd.arg(format!("file_{i}.txt"));
    }
    cmd.current_dir(temp_dir.path());

    let mut child = pty.spawn_command(&mut cmd).unwrap();
    sleep(Duration::from_millis(1000));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    Content 0                               \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "file_0.txt" [noeol] 1L, 9B             \n
    "#);

    pty.send_input_str(":bn\n").unwrap();
    sleep(Duration::from_millis(100));

    pty.send_input_str(":bp\n").unwrap();
    sleep(Duration::from_millis(100));

    let (row, _col) = pty.get_cursor_position();
    assert!(row < 10, "PTY cursor should be within bounds");

    pty.send_input_str(":qa!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_deep_undo_history_pty() {
    let temp_dir = TempDir::new().unwrap();

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            Command::new("vim")
                .arg("undo_test.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    pty.send_input_str("i").unwrap();
    for i in 0..20 {
        pty.send_input_str(&format!("Change {i} ")).unwrap();
        pty.send_input_str("\x1b").unwrap();
        pty.send_input_str("a").unwrap();
    }
    pty.send_input_str("\x1b").unwrap();
    sleep(Duration::from_millis(500));

    for _ in 0..10 {
        pty.send_input_str("u").unwrap();
        sleep(Duration::from_millis(10));
    }

    let snapshot = pty.get_snapshot();
    // Normalize time patterns using regex to handle all variations
    let time_regex = Regex::new(r"\d+\s+seconds?\s+ago").unwrap();
    let normalized_snapshot = time_regex
        .replace_all(&snapshot, |caps: &regex::Captures| {
            " ".repeat(caps.get(0).unwrap().as_str().len())
        })
        .to_string();
    insta::assert_snapshot!(normalized_snapshot, @r"
    Change 0 Change 1 Change 2 Change 3 Chan\n
    ge 4 Change 5 Change 6 Change 7 Change 8\n
     Change 9                               \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    1 change; before #11                    \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}
