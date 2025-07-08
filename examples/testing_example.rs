use virtual_tty::VirtualTty;

/// Example of how to test a CLI application using VirtualTty
/// This simulates testing a simple CLI app that writes to stdout/stderr

struct MyCliApp {
    name: String,
}

impl MyCliApp {
    fn new(name: String) -> Self {
        Self { name }
    }
    
    /// Simulate running the app - it writes to the provided TTY
    fn run(&self, tty: &mut VirtualTty) {
        tty.stdout_write(&format!("Welcome to {}!\n", self.name));
        tty.stdout_write("Please enter your name: ");
        
        // Simulate user input
        tty.send_input("Alice\n");
        
        // App responds
        tty.stdout_write(&format!("Hello, Alice! Welcome to {}.\n", self.name));
        
        // Simulate some processing with progress
        tty.stdout_write("Processing");
        for _ in 0..3 {
            std::thread::sleep(std::time::Duration::from_millis(10));
            tty.stdout_write(".");
        }
        tty.stdout_write(" Done!\n");
        
        // Show some colored output (ANSI codes)
        tty.stdout_write("\x1b[32mSuccess!\x1b[0m Operation completed.\n");
        
        // Show error message to stderr
        tty.stderr_write("\x1b[31mWarning:\x1b[0m This is just a demo.\n");
    }
}

fn main() {
    println!("Testing CLI Application with VirtualTty\n");
    
    // Create the app we want to test
    let app = MyCliApp::new("TestApp".to_string());
    
    // Create a virtual TTY for testing
    let mut tty = VirtualTty::new(60, 15);
    
    // Run the app in the virtual TTY
    app.run(&mut tty);
    
    // Get the complete output
    let output = tty.get_snapshot();
    
    println!("=== Application Output ===");
    println!("{}", output);
    println!("=========================\n");
    
    // Example of assertions you might do in a real test
    assert!(output.contains("Welcome to TestApp!"));
    assert!(output.contains("Hello, Alice!"));
    assert!(output.contains("Success!"));
    assert!(output.contains("Warning:"));
    
    println!("✅ All assertions passed!");
    
    // Show cursor position after execution
    let (row, col) = tty.get_cursor_position();
    println!("Final cursor position: row {}, col {}", row, col);
}