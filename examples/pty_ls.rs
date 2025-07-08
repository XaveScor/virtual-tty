use virtual_tty_pty::PtyAdapter;
use std::process::Command;

fn main() {
    // Create a PTY adapter with virtual TTY
    let mut pty = PtyAdapter::new(80, 24);
    
    // Create ls command with color output
    let mut cmd = Command::new("ls");
    cmd.args(&["-la", "--color=always"]);
    
    println!("Running 'ls -la --color=always' in virtual TTY...\n");
    
    // Spawn the command in the virtual TTY
    match pty.spawn_command(&mut cmd) {
        Ok(mut child) => {
            // Wait for the command to complete
            match child.wait() {
                Ok(status) => {
                    println!("Command exited with: {}", status);
                    
                    // Give the reader thread time to process all output
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    
                    // Get and print the terminal snapshot
                    println!("\n=== Virtual TTY Output ===");
                    println!("{}", pty.get_snapshot());
                    println!("=========================");
                }
                Err(e) => eprintln!("Failed to wait for command: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to spawn command: {}", e),
    }
}