use virtual_tty_pty::PtyAdapter;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a PTY adapter
    let mut pty = PtyAdapter::new(80, 24);
    
    // Example 1: Run a simple interactive command
    println!("Example 1: Running bash with echo commands...\n");
    
    let mut cmd = Command::new("bash");
    
    match pty.spawn_command(&mut cmd) {
        Ok(mut child) => {
            // Send some commands to bash
            thread::sleep(Duration::from_millis(100));
            
            // Send echo command
            let _ = pty.send_input_str("echo 'Hello from Virtual TTY!'\n");
            thread::sleep(Duration::from_millis(100));
            
            // Send another command
            let _ = pty.send_input_str("echo 'Testing input/output'\n");
            thread::sleep(Duration::from_millis(100));
            
            // Send pwd command
            let _ = pty.send_input_str("pwd\n");
            thread::sleep(Duration::from_millis(100));
            
            // Exit bash
            let _ = pty.send_input_str("exit\n");
            
            // Wait for bash to exit
            let _ = child.wait();
            thread::sleep(Duration::from_millis(200));
            
            // Get and print the terminal snapshot
            println!("=== Virtual TTY Output ===");
            println!("{}", pty.get_snapshot());
            println!("=========================\n");
        }
        Err(e) => eprintln!("Failed to spawn bash: {}", e),
    }
    
    // Example 2: Run a command that produces both stdout and stderr
    println!("Example 2: Command with stderr output...\n");
    
    let mut pty2 = PtyAdapter::new(80, 10);
    let mut cmd2 = Command::new("bash");
    cmd2.arg("-c").arg("echo 'This is stdout'; echo 'This is stderr' >&2; ls /nonexistent 2>&1");
    
    match pty2.spawn_command(&mut cmd2) {
        Ok(mut child) => {
            let _ = child.wait();
            thread::sleep(Duration::from_millis(100));
            
            println!("=== Mixed stdout/stderr ===");
            println!("{}", pty2.get_snapshot());
            println!("==========================");
        }
        Err(e) => eprintln!("Failed to spawn command: {}", e),
    }
}