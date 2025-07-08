use virtual_tty::VirtualTty;

fn main() {
    // Create a virtual TTY with 40x10 dimensions
    let mut tty = VirtualTty::new(40, 10);
    
    // Write some text
    tty.stdout_write("Hello, Virtual TTY!\n");
    tty.stdout_write("This is line 2.\n");
    
    // Use ANSI escape sequences
    tty.stdout_write("\x1b[1;1H"); // Move to top-left
    tty.stdout_write("*");
    
    // Write to stderr
    tty.stderr_write("\nError: This is an error message\n");
    
    // Clear a line
    tty.stdout_write("This line will be partially cleared");
    tty.stdout_write("\x1b[10D"); // Move back 10
    tty.stdout_write("\x1b[K"); // Clear to end of line
    
    // Get and print the snapshot
    println!("Terminal snapshot:");
    println!("==================");
    println!("{}", tty.get_snapshot());
    println!("==================");
    
    // Show terminal dimensions
    let (width, height) = tty.get_size();
    println!("Terminal size: {}x{}", width, height);
    
    // Show cursor position
    let (row, col) = tty.get_cursor_position();
    println!("Cursor position: row {}, col {}", row, col);
}