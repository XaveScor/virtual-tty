use std::process::Command;
use std::thread;
use std::time::Duration;
use virtual_tty_pty::PtyAdapter;

#[path = "common/mod.rs"]
mod common;
use common::*;

fn get_snapshots_dir() -> std::path::PathBuf {
    snapshots_dir_for_tool("less")
}

#[test]
fn test_less_hjkl_navigation() {
    ensure_test_files_exist();

    let output = run_pty_test(50, 8, |pty| {
        let mut cmd = Command::new("less");
        cmd.arg("numbered_lines.txt");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Give less time to start
        thread::sleep(Duration::from_millis(200));

        // Test basic navigation
        // Move down 3 lines with 'j'
        for _ in 0..3 {
            pty.send_input(b"j")?;
            thread::sleep(Duration::from_millis(50));
        }

        // Move up 2 lines with 'k'
        for _ in 0..2 {
            pty.send_input(b"k")?;
            thread::sleep(Duration::from_millis(50));
        }

        // Give time for final state
        thread::sleep(Duration::from_millis(100));

        // Quit less
        pty.send_input(b"q")?;
        let _ = child.wait();

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "hjkl_navigation", &output);
}

#[test]
fn test_less_page_navigation() {
    ensure_test_files_exist();

    let output = run_pty_test(60, 10, |pty| {
        let mut cmd = Command::new("less");
        cmd.arg("large_content.txt");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Give less time to start
        thread::sleep(Duration::from_millis(200));

        // Page down with space
        pty.send_input(b" ")?;
        thread::sleep(Duration::from_millis(150));

        // Page down again
        pty.send_input(b" ")?;
        thread::sleep(Duration::from_millis(150));

        // Page up with 'b'
        pty.send_input(b"b")?;
        thread::sleep(Duration::from_millis(150));

        // Quit less
        pty.send_input(b"q")?;
        let _ = child.wait();

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "page_navigation", &output);
}

#[test]
fn test_less_jump_commands() {
    ensure_test_files_exist();

    let output = run_pty_test(55, 12, |pty| {
        let mut cmd = Command::new("less");
        cmd.arg("numbered_lines.txt");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Give less time to start
        thread::sleep(Duration::from_millis(200));

        // Jump to end with 'G'
        pty.send_input(b"G")?;
        thread::sleep(Duration::from_millis(150));

        // Jump to beginning with 'g'
        pty.send_input(b"g")?;
        thread::sleep(Duration::from_millis(150));

        // Jump to line 50
        pty.send_input(b"50G")?;
        thread::sleep(Duration::from_millis(150));

        // Quit less
        pty.send_input(b"q")?;
        let _ = child.wait();

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "jump_commands", &output);
}

#[test]
fn test_less_search() {
    ensure_test_files_exist();

    let output = run_pty_test(60, 10, |pty| {
        let mut cmd = Command::new("less");
        cmd.arg("numbered_lines.txt");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Give less time to start
        thread::sleep(Duration::from_millis(200));

        // Search for "50"
        pty.send_input(b"/50\n")?;
        thread::sleep(Duration::from_millis(150));

        // Search next with 'n'
        pty.send_input(b"n")?;
        thread::sleep(Duration::from_millis(100));

        // Quit less
        pty.send_input(b"q")?;
        let _ = child.wait();

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "search", &output);
}
