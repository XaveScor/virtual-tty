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

#[test]
fn test_vim_coding_session_pty() {
    let temp_dir = TempDir::new().unwrap();

    // Create initial code file
    copy_fixture_to_dir(temp_dir.path(), "sample_code.rs", "main.rs");

    let mut pty = PtyAdapter::new(60, 15);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("main.rs")
                .current_dir(temp_dir.path()),
        )
        .unwrap();

    sleep(Duration::from_millis(1000));

    // Enable developer settings
    pty.send_input_str(":set number\n").unwrap();
    pty.send_input_str(":set expandtab\n").unwrap();
    pty.send_input_str(":set tabstop=4\n").unwrap();
    sleep(Duration::from_millis(300));

    // PTY should show line numbers and code content
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
      1 use std::collections::HashMap;                          \n
      2 use std::error::Error;                                  \n
      3                                                         \n
      4 #[derive(Debug)]                                        \n
      5 struct CustomError {                                    \n
      6     message: String,                                    \n
      7 }                                                       \n
      8                                                         \n
      9 impl std::fmt::Display for CustomError {                \n
     10     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::f\n
        mt::Result {                                            \n
     11         write!(f, "Custom error: {}", self.message)     \n
     12     }                                                   \n
     13 }                                                       \n
    :set tabstop=4                                              \n
    "#);

    // Navigate to end of main function and add new code
    pty.send_input_str("G").unwrap(); // Go to end
    pty.send_input_str("O").unwrap(); // Open line above

    let new_code = "    // Add error handling
    let result = process_data(&data);";

    pty.send_input_str(new_code).unwrap();
    pty.send_input_str("\x1b").unwrap(); // ESC
    sleep(Duration::from_millis(300));

    // Test save
    pty.send_input_str(":w\n").unwrap();
    sleep(Duration::from_millis(200));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
     26 fn process_data(data: &HashMap<&str, &str>) -> Result<()\n
        , CustomError> {                                        \n
     27     if data.is_empty() {                                \n
     28         return Err(CustomError {                        \n
     29             message: "Data cannot be empty".to_string(),\n
     30         });                                             \n
     31     }                                                   \n
     32                                                         \n
     33     for (key, value) in data {                          \n
     34         println!("{}: {}", key, value);                 \n
     35     }                                                   \n
     36     // Add error handling                               \n
     39     let result = process_data(&data);                   \n
     40 }                                                       \n
    "main.rs" 40L, 881B written                                 \n
    "#);

    pty.send_input_str(":q\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_large_file_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "large_file.txt", "large.txt");

    let start_time = std::time::Instant::now();

    let mut pty = PtyAdapter::new(50, 12);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("large.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();

    sleep(Duration::from_millis(2000));

    let load_time = start_time.elapsed();
    assert!(
        load_time < Duration::from_secs(3),
        "PTY should handle large file loading within 3 seconds"
    );

    // Test navigation in large file
    pty.send_input_str("G").unwrap(); // Go to end
    sleep(Duration::from_millis(200));

    let (row, _) = pty.get_cursor_position();
    assert!(row >= 10, "PTY should navigate to end of large file");

    // Test searching
    pty.send_input_str("/Line 50\n").unwrap();
    sleep(Duration::from_millis(300));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line 45: Signal handling robustness               \n
    Line 46: Exception safety guarantees              \n
    Line 47: Data integrity maintenance               \n
    Line 48: Consistency across operations            \n
    Line 49: Reliability under stress                 \n
    Line 50: Performance benchmarking baseline        \n
    Line 51: Continuation of performance testing conte\n
    nt                                                \n
    Line 52: Extended content for thorough validation \n
    Line 53: Additional lines for comprehensive testin\n
    g                                                 \n
    search hit BOTTOM, continuing at TOP              \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_project_navigation_pty() {
    let temp_dir = TempDir::new().unwrap();

    // Create project structure
    fs::create_dir_all(temp_dir.path().join("src")).unwrap();
    fs::create_dir_all(temp_dir.path().join("tests")).unwrap();

    // Create project files
    copy_fixture_to_dir(temp_dir.path(), "sample_code.rs", "src/main.rs");
    copy_fixture_to_dir(temp_dir.path(), "basic_content.txt", "src/lib.rs");
    copy_fixture_to_dir(temp_dir.path(), "multiline_content.txt", "tests/test.rs");

    let mut pty = PtyAdapter::new(50, 12);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("src/main.rs")
                .current_dir(temp_dir.path()),
        )
        .unwrap();

    sleep(Duration::from_millis(500));

    // Open additional files
    pty.send_input_str(":e src/lib.rs\n").unwrap();
    sleep(Duration::from_millis(200));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    Line 1                                            \n
    Line 2                                            \n
    Line 3                                            \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    "src/lib.rs" [noeol] 3L, 22B                      \n
    "#);

    // Test buffer navigation
    pty.send_input_str(":ls\n").unwrap();
    sleep(Duration::from_millis(200));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    :ls                                               \n
      1 #    "src/main.rs"                  line 1    \n
      2 %a   "src/lib.rs"                   line 1    \n
    Press ENTER or type command to continue           \n
    "#);

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_configuration_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "test_vimrc", "vimrc");
    copy_fixture_to_dir(temp_dir.path(), "basic_content.txt", "config_test.txt");

    let mut pty = PtyAdapter::new(50, 12);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .args(&["-u", "vimrc", "config_test.txt"])
                .current_dir(temp_dir.path()),
        )
        .unwrap();

    sleep(Duration::from_millis(1000));

    // PTY should apply configuration
    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
                                                      \n
                                                      \n
                                                      \n
                                                      \n
                                                      \n
                                                      \n
                                                      \n
                                                      \n
                                                      \n
    "config_test.txt" [Incomplete last line] 3 lines, \n
    22 bytes                                          \n
    Press ENTER or type command to continue           \n
    "#);

    // Test dynamic configuration changes
    pty.send_input_str(":set nonumber\n").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line 1                                            \n
    Line 2                                            \n
    Line 3                                            \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
                                                      \n
    ");

    pty.send_input_str(":set number\n").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
          1 Line 1                                    \n
          2 Line 2                                    \n
          3 Line 3                                    \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    :set number                                       \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_debugging_session_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "debug_test.js", "debug.js");

    let mut pty = PtyAdapter::new(50, 12);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("debug.js")
                .current_dir(temp_dir.path()),
        )
        .unwrap();

    sleep(Duration::from_millis(500));

    // Test handling of problematic content
    pty.send_input_str(":set number\n").unwrap();
    sleep(Duration::from_millis(100));

    // Navigate to problematic line
    pty.send_input_str("2G").unwrap();
    sleep(Duration::from_millis(100));

    let (row, _) = pty.get_cursor_position();
    assert_eq!(row, 1, "PTY should navigate to line 2");

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r#"
      1 function broken() {                           \n
      2     if (condition {  // Missing closing parent\n
        hesis                                         \n
      3         console.log("test"                    \n
      4     }                                         \n
      5     return undefined;                         \n
      6 }                                             \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    :set number                                       \n
    "#);

    // Test undo with edits
    pty.send_input_str("2G$a)\n").unwrap();
    pty.send_input_str("\x1b").unwrap();
    sleep(Duration::from_millis(100));

    pty.send_input_str("u").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    // Normalize the timing to handle variable timing
    let normalized_snapshot = snapshot
        .replace("1 second ago", "X seconds ago")
        .replace("0 seconds ago", "X seconds ago");
    insta::assert_snapshot!(normalized_snapshot, @r#"
      1 function broken() {                           \n
      2     if (condition {  // Missing closing parent\n
        hesis)                                        \n
      3         console.log("test"                    \n
      4     }                                         \n
      5     return undefined;                         \n
      6 }                                             \n
      7                                               \n
    ~                                                 \n
    ~                                                 \n
    ~                                                 \n
    1 line less; before #1  X seconds ago             \n
    "#);

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_documentation_session_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "documentation.md", "README.md");

    let mut pty = PtyAdapter::new(50, 12);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("README.md")
                .current_dir(temp_dir.path()),
        )
        .unwrap();

    sleep(Duration::from_millis(500));

    // Enable text wrapping for documentation
    pty.send_input_str(":set wrap\n").unwrap();
    pty.send_input_str(":set linebreak\n").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    # Project Documentation                           \n
                                                      \n
    This is a comprehensive guide to using the        \n
    project. It includes detailed explanations of all \n
    features and provides examples for common use     \n
    cases.                                            \n
                                                      \n
    ## Features                                       \n
                                                      \n
    1. **Feature One**: This feature does something   \n
    important and useful for users.                   \n
    :set linebreak                                    \n
    ");

    // Test navigation and formatting
    pty.send_input_str("gg").unwrap();
    pty.send_input_str("/Feature One\n").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    # Project Documentation                           \n
                                                      \n
    This is a comprehensive guide to using the        \n
    project. It includes detailed explanations of all \n
    features and provides examples for common use     \n
    cases.                                            \n
                                                      \n
    ## Features                                       \n
                                                      \n
    1. **Feature One**: This feature does something   \n
    important and useful for users.                   \n
    /Feature One                                      \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}

#[test]
fn test_vim_performance_stress_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "stress_test.txt", "stress.txt");

    let start_time = std::time::Instant::now();

    let mut pty = PtyAdapter::new(60, 15);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("stress.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();

    sleep(Duration::from_millis(1000));

    // Performance test: rapid navigation
    let nav_start = std::time::Instant::now();
    for _ in 0..10 {
        pty.send_input_str("G").unwrap();
        pty.send_input_str("gg").unwrap();
        sleep(Duration::from_millis(10));
    }
    let nav_time = nav_start.elapsed();

    assert!(
        nav_time < Duration::from_secs(2),
        "PTY should handle rapid navigation within 2 seconds"
    );

    // Performance test: rapid editing
    let edit_start = std::time::Instant::now();
    pty.send_input_str("gg").unwrap();
    for i in 0..10 {
        pty.send_input_str(&format!("A [{}]\x1b", i)).unwrap();
        pty.send_input_str("j").unwrap();
        sleep(Duration::from_millis(10));
    }
    let edit_time = edit_start.elapsed();

    assert!(
        edit_time < Duration::from_secs(3),
        "PTY should handle rapid editing within 3 seconds"
    );

    // Verify PTY state consistency
    let (row, col) = pty.get_cursor_position();
    assert!(row < 100, "PTY cursor should be within file bounds");
    assert!(col < 200, "PTY cursor column should be reasonable");

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Line 1: Lorem ipsum dolor sit amet, consectetur adipiscing e\n
    lit. Sed do eiusmod tempor incididunt ut labore et dolore ma\n
    gna aliqua. [0]                                             \n
    lit. 2: Lorem ipsum dolor sit amet, consectetur adipiscing e\n
    lit. Sed do eiusmod tempor incididunt ut labore et dolore ma\n
    gna aliqua. [1]                                             \n
    lit. 3: Lorem ipsum dolor sit amet, consectetur adipiscing e\n
    lit. Sed do eiusmod tempor incididunt ut labore et dolore ma\n
    gna aliqua. [[9]                                            \n
    Line 11: Lorem ipsum dolor sit amet, consectetur adipiscing \n
    elit. Sed do eiusmod tempor incididunt ut labore et dolore m\n
    agna aliqua.[[9]                                            \n
    @                                                           \n
    @                                                           \n
                                                                \n
    ");

    pty.send_input_str(":q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();

    let total_time = start_time.elapsed();
    assert!(
        total_time < Duration::from_secs(10),
        "PTY should complete stress test within 10 seconds"
    );
}
