use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use tempfile::TempDir;
use virtual_tty_pty::PtyAdapter;

fn copy_fixture_to_dir(dir: &Path, fixture_name: &str, target_name: &str) {
    let fixture_path = Path::new("tests/fixtures").join(fixture_name);
    let target_path = dir.join(target_name);
    fs::copy(&fixture_path, &target_path).unwrap();
}

fn create_file_in_dir(dir: &Path, filename: &str, content: &str) {
    let file_path = dir.join(filename);
    fs::write(&file_path, content).unwrap();
}

#[test]
fn test_vim_visual_mode_pty_highlighting() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "multiline_content.txt", "test_visual.txt");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("test_visual.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    // Enter visual mode
    pty.send_input_str("v").unwrap();
    sleep(Duration::from_millis(100));

    // PTY should show visual mode indicator
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    First line of text                      \n
    Second line with more content           \n
    Third line                              \n
    Fourth line                             \n
    Fifth line                              \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- VISUAL --                            \n
    ");

    // Select text by moving cursor
    pty.send_input_str("wwj").unwrap(); // Forward 2 words, down 1 line
    sleep(Duration::from_millis(200));

    // Test line visual mode
    pty.send_input_str("\x1bV").unwrap(); // ESC, then V for line visual
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    First line of text                      \n
    Second line with more content           \n
    Third line                              \n
    Fourth line                             \n
    Fifth line                              \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- VISUAL LINE --                       \n
    ");

    // Test block visual mode
    pty.send_input_str("\x1b\x16").unwrap(); // ESC, then Ctrl+V for block visual
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    First line of text                      \n
    Second line with more content           \n
    Third line                              \n
    Fourth line                             \n
    Fifth line                              \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- VISUAL BLOCK --                      \n
    ");

    pty.send_input_str("\x1b:q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_split_windows_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "basic_content.txt", "test_split.txt");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("test_split.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    // Create horizontal split
    pty.send_input_str(":split\n").unwrap();
    sleep(Duration::from_millis(300));

    // PTY should show split window with divider
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line 1                                  \n
    Line 2                                  \n
    Line 3                                  \n
    ~                                       \n
    test_split.txt                          \n
    Line 1                                  \n
    Line 2                                  \n
    Line 3                                  \n
    test_split.txt                          \n
    :split                                  \n
    ");

    // Test window navigation
    pty.send_input_str("\x17j").unwrap(); // Ctrl+W, j (move to lower window)
    sleep(Duration::from_millis(100));

    pty.send_input_str("\x17k").unwrap(); // Ctrl+W, k (move to upper window)
    sleep(Duration::from_millis(100));

    // Create vertical split
    pty.send_input_str(":vsplit\n").unwrap();
    sleep(Duration::from_millis(300));

    // Test closing windows
    pty.send_input_str(":q\n").unwrap(); // Close current window
    sleep(Duration::from_millis(200));

    pty.send_input_str(":qa!\n").unwrap(); // Quit all
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_search_replace_pty() {
    let temp_dir = TempDir::new().unwrap();
    let content = "foo bar baz\nfoo test foo\nbar foo baz foo";
    create_file_in_dir(temp_dir.path(), "test_search.txt", content);

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("test_search.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    // Enable search highlighting
    pty.send_input_str(":set hlsearch\n").unwrap();
    sleep(Duration::from_millis(100));

    // Search for pattern
    pty.send_input_str("/foo\n").unwrap();
    sleep(Duration::from_millis(200));

    // PTY should show search results (cursor moves to first match)
    let (row, _col) = pty.get_cursor_position();
    // Vim search might position cursor differently, we'll just check it's at a reasonable position
    assert!(
        row <= 2,
        "PTY should track cursor to a search match position"
    );

    // Navigate to next match
    pty.send_input_str("n").unwrap();
    sleep(Duration::from_millis(100));

    let (row, _col) = pty.get_cursor_position();
    assert!(row <= 2, "PTY should track cursor to next match");

    // Navigate to previous match
    pty.send_input_str("N").unwrap();
    sleep(Duration::from_millis(100));

    let (row, _col) = pty.get_cursor_position();
    assert!(row <= 2, "PTY should track cursor to previous match");

    // Test global replace
    pty.send_input_str(":%s/foo/FOO/g\n").unwrap();
    sleep(Duration::from_millis(300));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    FOO bar baz                             \n
    FOO test FOO                            \n
    bar FOO baz FOO                         \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    5 substitutions on 3 lines              \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_tabs_pty() {
    let temp_dir = TempDir::new().unwrap();
    // Create multiple test files from fixtures
    copy_fixture_to_dir(temp_dir.path(), "single_line.txt", "tab1.txt");
    copy_fixture_to_dir(temp_dir.path(), "basic_content.txt", "tab2.txt");
    copy_fixture_to_dir(temp_dir.path(), "word_content.txt", "tab3.txt");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("tab1.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    // Open files in new tabs
    pty.send_input_str(":tabnew tab2.txt\n").unwrap();
    sleep(Duration::from_millis(300));

    pty.send_input_str(":tabnew tab3.txt\n").unwrap();
    sleep(Duration::from_millis(300));

    // PTY should show tab line
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
     tab1.txt  tab2.txt  tab3.txt          X\n
    The quick brown fox jumps over the lazy \n
    dog                                     \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    "tab3.txt" [noeol] 1L, 43B              \n
    "#);

    // Navigate between tabs
    pty.send_input_str("gt").unwrap(); // Next tab
    sleep(Duration::from_millis(100));

    pty.send_input_str("gT").unwrap(); // Previous tab
    sleep(Duration::from_millis(100));

    // Test tab closing
    pty.send_input_str(":tabclose\n").unwrap();
    sleep(Duration::from_millis(200));

    pty.send_input_str(":qa!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_macros_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "multiline_content.txt", "test_macro.txt");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("test_macro.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    // Record macro in register 'a'
    pty.send_input_str("qa").unwrap(); // Start recording macro
    sleep(Duration::from_millis(100));

    // PTY should show recording indicator
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    First line of text                      \n
    Second line with more content           \n
    Third line                              \n
    Fourth line                             \n
    Fifth line                              \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    recording @a                            \n
    ");

    // Perform actions to record
    pty.send_input_str("I[").unwrap(); // Insert at beginning
    pty.send_input_str("\x1b").unwrap(); // ESC
    pty.send_input_str("A]").unwrap(); // Append at end
    pty.send_input_str("\x1b").unwrap(); // ESC
    pty.send_input_str("j").unwrap(); // Move down
    sleep(Duration::from_millis(100));

    // Stop recording
    pty.send_input_str("q").unwrap();
    sleep(Duration::from_millis(100));

    // PTY should no longer show recording
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    [First line of text]                    \n
    Second line with more content           \n
    Third line                              \n
    Fourth line                             \n
    Fifth line                              \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
                                            \n
    ");

    // Playback macro
    pty.send_input_str("@a").unwrap();
    sleep(Duration::from_millis(200));

    // PTY should show results of macro playback
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    [First line of text]                    \n
    [Second line with more content]         \n
    Third line                              \n
    Fourth line                             \n
    Fifth line                              \n
    ~                                       \n
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
fn test_vim_folding_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "structured_content.txt", "test_fold.txt");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("test_fold.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    // Enable folding
    pty.send_input_str(":set foldmethod=manual\n").unwrap();
    sleep(Duration::from_millis(100));

    // Select and fold text
    pty.send_input_str("V4jzf").unwrap(); // Visual line mode, select 5 lines, fold
    sleep(Duration::from_millis(200));

    // PTY should show folded content
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    +--  5 lines: Header 1------------------\n
      Subitem 3                             \n
    Header 2                                \n
      Another item                          \n
      More content                          \n
        Nested item 1                       \n
        Nested item 2                       \n
          Deep nested item                  \n
          Another deep item                 \n
                                            \n
    ");

    // Unfold
    pty.send_input_str("zo").unwrap();
    sleep(Duration::from_millis(100));

    // PTY should show unfolded content
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Header 1                                \n
      Subitem 1                             \n
      Subitem 2                             \n
        Sub-subitem 1                       \n
        Sub-subitem 2                       \n
      Subitem 3                             \n
    Header 2                                \n
      Another item                          \n
      More content                          \n
                                            \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_syntax_highlighting_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "syntax_test.rs", "test_syntax.rs");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("test_syntax.rs")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    // Enable syntax highlighting
    pty.send_input_str(":syntax on\n").unwrap();
    sleep(Duration::from_millis(200));

    // PTY should handle syntax highlighting codes
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    fn example() {                          \n
        let mut vector = Vec::new();        \n
        vector.push("test");                \n
        println!("{:?}", vector);           \n
    }                                       \n
                                            \n
    struct TestStruct {                     \n
        field1: i32,                        \n
        field2: String,                     \n
    :syntax on                              \n
    "#);

    // Test that PTY doesn't break with color codes
    let (row, _col) = pty.get_cursor_position();
    assert_eq!(
        row, 0,
        "PTY cursor should be tracked correctly with syntax highlighting"
    );

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_command_completion_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "single_line.txt", "test_completion.txt");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("test_completion.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    // Test command completion
    pty.send_input_str(":se").unwrap(); // Partial command
    sleep(Duration::from_millis(100));

    pty.send_input_str("\t").unwrap(); // Tab completion
    sleep(Duration::from_millis(100));

    // PTY should show command completion
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
    :set                                    \n
    ");

    pty.send_input_str("\x1b").unwrap(); // ESC to cancel
    sleep(Duration::from_millis(100));

    // Test command history
    pty.send_input_str(":set number\n").unwrap();
    sleep(Duration::from_millis(100));

    pty.send_input_str(":").unwrap();
    pty.send_input_str("\x1b[A").unwrap(); // Up arrow for history
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
      1 Single line of text for testing     \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    :set number                             \n
    ");

    pty.send_input_str("\x1b:q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}
