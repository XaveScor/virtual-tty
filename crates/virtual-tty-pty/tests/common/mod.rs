use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use virtual_tty_pty::PtyAdapter;

/// Get the snapshots directory for a specific tool on the current platform
pub fn snapshots_dir_for_tool(tool: &str) -> PathBuf {
    let platform = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        panic!("Unsupported platform for PTY tests")
    };

    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(tool)
        .join("snapshots")
        .join(platform)
}

/// Assert that output matches a snapshot, or create/update the snapshot
pub fn assert_snapshot_matches(snapshots_dir: &Path, name: &str, actual: &str) {
    let snapshot_path = snapshots_dir.join(format!("{}.snap", name));

    // Ensure the snapshots directory exists
    if let Some(parent) = snapshot_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create snapshots directory");
    }

    // Normalize the output for consistent snapshots
    let normalized_actual = normalize_output(actual);

    match fs::read_to_string(&snapshot_path) {
        Ok(expected) => {
            let normalized_expected = normalize_output(&expected);
            if normalized_actual != normalized_expected {
                // Print diff for debugging
                println!("Snapshot mismatch for {}:", name);
                println!("Expected:\n{}", normalized_expected);
                println!("Actual:\n{}", normalized_actual);

                // Check if we should update snapshots (via environment variable)
                if std::env::var("UPDATE_SNAPSHOTS").is_ok() {
                    fs::write(&snapshot_path, &normalized_actual)
                        .expect("Failed to write updated snapshot");
                    println!("Updated snapshot: {}", snapshot_path.display());
                } else {
                    panic!("Snapshot mismatch. Set UPDATE_SNAPSHOTS=1 to update.");
                }
            }
        }
        Err(_) => {
            // Snapshot doesn't exist, create it
            fs::write(&snapshot_path, &normalized_actual).expect("Failed to write new snapshot");
            println!("Created new snapshot: {}", snapshot_path.display());
        }
    }
}

/// Normalize output to make snapshots more stable across runs
fn normalize_output(output: &str) -> String {
    output
        .lines()
        .map(|line| line.trim_end()) // Remove trailing whitespace
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end() // Remove trailing newlines
        .to_string()
}

/// Create a platform-specific command for common tools
pub fn create_platform_command(tool: &str) -> Command {
    match tool {
        "ls" => {
            let mut cmd = Command::new("ls");
            #[cfg(target_os = "linux")]
            cmd.args(&["-la", "--color=always"]);

            #[cfg(target_os = "macos")]
            cmd.args(&["-laG"]);

            cmd
        }
        "less" => Command::new("less"),
        "vim" => Command::new("vim"),
        _ => panic!("Unsupported tool: {}", tool),
    }
}

/// Run a PTY test with a given command and return the snapshot
pub fn run_pty_test<F>(width: usize, height: usize, setup_and_run: F) -> String
where
    F: FnOnce(&mut PtyAdapter) -> Result<(), Box<dyn std::error::Error>>,
{
    let mut pty = PtyAdapter::new(width, height);

    match setup_and_run(&mut pty) {
        Ok(_) => pty.get_snapshot(),
        Err(e) => {
            eprintln!("PTY test failed: {}", e);
            // Return the snapshot anyway for debugging
            pty.get_snapshot()
        }
    }
}

/// Helper to wait for a command to complete and get output
pub fn wait_for_output(pty: &PtyAdapter, duration_ms: u64) -> String {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    pty.get_snapshot()
}

/// Generate test content files if they don't exist
pub fn ensure_test_files_exist() {
    use std::io::Write;

    // Generate numbered_lines.txt if it doesn't exist
    if !Path::new("numbered_lines.txt").exists() {
        let mut file = fs::File::create("numbered_lines.txt").expect("Failed to create test file");
        for i in 1..=100 {
            writeln!(
                file,
                "Line {:3}: This is line number {} with some additional content",
                i, i
            )
            .expect("Failed to write to test file");
        }
    }

    // Generate large_content.txt if it doesn't exist
    if !Path::new("large_content.txt").exists() {
        let mut file = fs::File::create("large_content.txt").expect("Failed to create test file");
        writeln!(file, "LARGE CONTENT FILE FOR TESTING").expect("Failed to write to test file");
        writeln!(file, "================================").expect("Failed to write to test file");
        writeln!(file).expect("Failed to write to test file");

        for section in 1..=5 {
            writeln!(file, "SECTION {}", section).expect("Failed to write to test file");
            writeln!(file, "----------").expect("Failed to write to test file");
            for i in 1..=10 {
                writeln!(
                    file,
                    "Section {} line {}: Content for testing scrolling behavior",
                    section, i
                )
                .expect("Failed to write to test file");
            }
            writeln!(file).expect("Failed to write to test file");
        }
    }
}
