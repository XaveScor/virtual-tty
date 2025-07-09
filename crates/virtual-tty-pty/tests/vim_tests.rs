use std::process::Command;
use std::thread;
use std::time::Duration;
use virtual_tty_pty::PtyAdapter;

#[path = "common/mod.rs"]
mod common;
use common::*;

fn get_snapshots_dir() -> std::path::PathBuf {
    snapshots_dir_for_tool("vim")
}

#[test]
fn test_vim_basic_movement() {
    ensure_test_files_exist();

    let output = run_pty_test(45, 10, |pty| {
        let mut cmd = Command::new("vim");
        cmd.arg("large_content.txt");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Give vim time to start
        thread::sleep(Duration::from_millis(300));

        // Test basic hjkl movement
        // Move to line start, then right 5 times
        pty.send_input(b"0")?;
        thread::sleep(Duration::from_millis(50));

        for _ in 0..5 {
            pty.send_input(b"l")?; // right
            thread::sleep(Duration::from_millis(25));
        }

        // Move left 3 times
        for _ in 0..3 {
            pty.send_input(b"h")?; // left
            thread::sleep(Duration::from_millis(25));
        }

        // Move down 5 times
        for _ in 0..5 {
            pty.send_input(b"j")?; // down
            thread::sleep(Duration::from_millis(25));
        }

        // Move up 3 times
        for _ in 0..3 {
            pty.send_input(b"k")?; // up
            thread::sleep(Duration::from_millis(25));
        }

        // Final state
        thread::sleep(Duration::from_millis(100));

        // Quit vim
        pty.send_input(b"\x1b:q!\n")?;
        let _ = child.wait();

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "basic_movement", &output);
}

#[test]
fn test_vim_scrolling() {
    ensure_test_files_exist();

    let output = run_pty_test(50, 8, |pty| {
        let mut cmd = Command::new("vim");
        cmd.arg("numbered_lines.txt");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Give vim time to start
        thread::sleep(Duration::from_millis(300));

        // Scroll down half page (Ctrl+D)
        pty.send_input(b"\x04")?; // Ctrl+D
        thread::sleep(Duration::from_millis(150));

        // Scroll down again
        pty.send_input(b"\x04")?;
        thread::sleep(Duration::from_millis(150));

        // Scroll up half page (Ctrl+U)
        pty.send_input(b"\x15")?; // Ctrl+U
        thread::sleep(Duration::from_millis(150));

        // Page down (Ctrl+F)
        pty.send_input(b"\x06")?; // Ctrl+F
        thread::sleep(Duration::from_millis(150));

        // Page up (Ctrl+B)
        pty.send_input(b"\x02")?; // Ctrl+B
        thread::sleep(Duration::from_millis(150));

        // Quit vim
        pty.send_input(b"\x1b:q!\n")?;
        let _ = child.wait();

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "scrolling", &output);
}

#[test]
fn test_vim_jump_commands() {
    ensure_test_files_exist();

    let output = run_pty_test(55, 12, |pty| {
        let mut cmd = Command::new("vim");
        cmd.arg("numbered_lines.txt");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Give vim time to start
        thread::sleep(Duration::from_millis(300));

        // Jump to end of file
        pty.send_input(b"G")?;
        thread::sleep(Duration::from_millis(150));

        // Jump to beginning
        pty.send_input(b"gg")?;
        thread::sleep(Duration::from_millis(150));

        // Jump to line 50
        pty.send_input(b"50G")?;
        thread::sleep(Duration::from_millis(150));

        // Jump to line 25 using :25
        pty.send_input(b":25\n")?;
        thread::sleep(Duration::from_millis(150));

        // Quit vim
        pty.send_input(b"\x1b:q!\n")?;
        let _ = child.wait();

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "jump_commands", &output);
}

#[test]
fn test_vim_word_movement() {
    ensure_test_files_exist();

    let output = run_pty_test(60, 10, |pty| {
        let mut cmd = Command::new("vim");
        cmd.arg("large_content.txt");

        let mut child = pty.spawn_command(&mut cmd)?;

        // Give vim time to start
        thread::sleep(Duration::from_millis(300));

        // Test word movement
        for _ in 0..3 {
            pty.send_input(b"w")?; // next word
            thread::sleep(Duration::from_millis(50));
        }

        // Move back words
        for _ in 0..2 {
            pty.send_input(b"b")?; // previous word
            thread::sleep(Duration::from_millis(50));
        }

        // End of line
        pty.send_input(b"$")?;
        thread::sleep(Duration::from_millis(100));

        // Beginning of line
        pty.send_input(b"0")?;
        thread::sleep(Duration::from_millis(100));

        // Quit vim
        pty.send_input(b"\x1b:q!\n")?;
        let _ = child.wait();

        Ok(())
    });

    assert_snapshot_matches(&get_snapshots_dir(), "word_movement", &output);
}
