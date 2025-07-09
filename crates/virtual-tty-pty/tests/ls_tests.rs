use std::thread;
use std::time::Duration;
use virtual_tty_pty::PtyAdapter;

#[path = "common/mod.rs"]
mod common;
use common::*;

fn get_snapshots_dir() -> std::path::PathBuf {
    snapshots_dir_for_tool("ls")
}

#[test]
fn test_ls_colored_output() {
    let output = run_pty_test(80, 24, |pty| {
        let mut cmd = create_platform_command("ls");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Wait for ls to complete
        let status = child.wait()?;

        // Give time for output to be processed
        thread::sleep(Duration::from_millis(100));

        if !status.success() {
            return Err(format!("ls command failed with status: {}", status).into());
        }

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "colored_output", &output);
}

#[test]
fn test_ls_with_specific_files() {
    ensure_test_files_exist();

    let output = run_pty_test(100, 15, |pty| {
        let mut cmd = create_platform_command("ls");
        // Add the generated test files to the ls command
        cmd.args(&["numbered_lines.txt", "large_content.txt"]);

        let mut child = pty.spawn_command(&mut cmd)?;

        // Wait for ls to complete
        let status = child.wait()?;

        // Give time for output to be processed
        thread::sleep(Duration::from_millis(100));

        if !status.success() {
            return Err(format!("ls command failed with status: {}", status).into());
        }

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "specific_files", &output);
}

#[test]
fn test_ls_long_output_in_small_window() {
    let output = run_pty_test(40, 8, |pty| {
        let mut cmd = create_platform_command("ls");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Wait for ls to complete
        let status = child.wait()?;

        // Give time for output to be processed
        thread::sleep(Duration::from_millis(100));

        if !status.success() {
            return Err(format!("ls command failed with status: {}", status).into());
        }

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "small_window", &output);
}

#[test]
fn test_ls_nonexistent_file() {
    let output = run_pty_test(60, 10, |pty| {
        let mut cmd = create_platform_command("ls");
        cmd.arg("nonexistent_file_12345.txt");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Wait for ls to complete (this should fail)
        let _status = child.wait(); // Don't check status as this should fail

        // Give time for error output to be processed
        thread::sleep(Duration::from_millis(100));

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "error_output", &output);
}
