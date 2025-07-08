use virtual_tty_pty::PtyAdapter;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Testing scrolling behavior with 'less' in a small terminal window\n");
    
    // Generate test content first if it doesn't exist
    println!("🔧 Generating test content...");
    let _ = Command::new("cargo")
        .args(&["run", "--example", "generate_test_content"])
        .output();
    
    // Test 1: Basic less navigation with hjkl
    test_less_basic_navigation();
    
    // Test 2: Page navigation
    test_less_page_navigation();
    
    // Test 3: Jump to specific locations
    test_less_jump_navigation();
    
    println!("\n✅ All less scrolling tests completed!");
}

fn test_less_basic_navigation() {
    println!("\n📋 Test 1: Basic hjkl navigation in less");
    println!("Terminal size: 50x8 (very small to force scrolling)");
    
    // Create a small terminal window that will force scrolling
    let mut pty = PtyAdapter::new(50, 8);
    
    // Start less with the numbered lines file
    let mut cmd = Command::new("less");
    cmd.arg("numbered_lines.txt");
    
    match pty.spawn_command(&mut cmd) {
        Ok(mut child) => {
            // Give less time to start and display content
            thread::sleep(Duration::from_millis(200));
            
            println!("Initial state:");
            print_snapshot(&pty, "📄");
            
            // Test basic navigation
            println!("\n🔽 Pressing 'j' (down) 3 times...");
            for _ in 0..3 {
                let _ = pty.send_input(b"j");
                thread::sleep(Duration::from_millis(100));
            }
            print_snapshot(&pty, "⬇️");
            
            println!("\n🔼 Pressing 'k' (up) 2 times...");
            for _ in 0..2 {
                let _ = pty.send_input(b"k");
                thread::sleep(Duration::from_millis(100));
            }
            print_snapshot(&pty, "⬆️");
            
            // Quit less
            let _ = pty.send_input(b"q");
            let _ = child.wait();
        }
        Err(e) => eprintln!("❌ Failed to start less: {}", e),
    }
}

fn test_less_page_navigation() {
    println!("\n📋 Test 2: Page navigation (space/b keys)");
    
    let mut pty = PtyAdapter::new(60, 10);
    
    let mut cmd = Command::new("less");
    cmd.arg("large_content.txt");
    
    match pty.spawn_command(&mut cmd) {
        Ok(mut child) => {
            thread::sleep(Duration::from_millis(200));
            
            println!("Initial page:");
            print_snapshot(&pty, "📄");
            
            // Page down
            println!("\n⬇️ Pressing SPACE (page down)...");
            let _ = pty.send_input(b" ");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "📄");
            
            // Page down again
            println!("\n⬇️ Pressing SPACE again...");
            let _ = pty.send_input(b" ");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "📄");
            
            // Page up
            println!("\n⬆️ Pressing 'b' (page up)...");
            let _ = pty.send_input(b"b");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "📄");
            
            // Quit
            let _ = pty.send_input(b"q");
            let _ = child.wait();
        }
        Err(e) => eprintln!("❌ Failed to start less: {}", e),
    }
}

fn test_less_jump_navigation() {
    println!("\n📋 Test 3: Jump navigation (G, g commands)");
    
    let mut pty = PtyAdapter::new(55, 12);
    
    let mut cmd = Command::new("less");
    cmd.arg("numbered_lines.txt");
    
    match pty.spawn_command(&mut cmd) {
        Ok(mut child) => {
            thread::sleep(Duration::from_millis(200));
            
            println!("Starting at beginning:");
            print_snapshot(&pty, "🏠");
            
            // Jump to end
            println!("\n🔚 Pressing 'G' (go to end)...");
            let _ = pty.send_input(b"G");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "🔚");
            
            // Jump to beginning
            println!("\n🏠 Pressing 'g' (go to beginning)...");
            let _ = pty.send_input(b"g");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "🏠");
            
            // Jump to specific line (line 50)
            println!("\n🎯 Going to line 50 (typing '50G')...");
            let _ = pty.send_input(b"50G");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "🎯");
            
            // Quit
            let _ = pty.send_input(b"q");
            let _ = child.wait();
        }
        Err(e) => eprintln!("❌ Failed to start less: {}", e),
    }
}

fn print_snapshot(pty: &PtyAdapter, prefix: &str) {
    let snapshot = pty.get_snapshot();
    let lines: Vec<&str> = snapshot.lines().collect();
    
    println!("{} Current view ({} lines visible):", prefix, lines.len());
    println!("┌{}┐", "─".repeat(52));
    for (i, line) in lines.iter().take(12).enumerate() {
        let truncated = if line.len() > 50 {
            format!("{}...", &line[..47])
        } else {
            format!("{:<50}", line)
        };
        println!("│{}│", truncated);
    }
    println!("└{}┘", "─".repeat(52));
}