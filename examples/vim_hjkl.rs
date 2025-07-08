use virtual_tty_pty::PtyAdapter;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Testing vim hjkl navigation and scrolling in a small terminal window\n");
    
    // Generate test content first if it doesn't exist
    println!("🔧 Generating test content...");
    let _ = Command::new("cargo")
        .args(&["run", "--example", "generate_test_content"])
        .output();
    
    // Test 1: Basic hjkl movement
    test_vim_basic_movement();
    
    // Test 2: Scrolling with Ctrl+D and Ctrl+U
    test_vim_scrolling();
    
    // Test 3: Jump commands (G, gg, line numbers)
    test_vim_jump_commands();
    
    println!("\n✅ All vim navigation tests completed!");
}

fn test_vim_basic_movement() {
    println!("\n📋 Test 1: Basic hjkl movement in vim");
    println!("Terminal size: 45x10 (small window)");
    
    let mut pty = PtyAdapter::new(45, 10);
    
    // Start vim with the large content file
    let mut cmd = Command::new("vim");
    cmd.arg("large_content.txt");
    
    match pty.spawn_command(&mut cmd) {
        Ok(mut child) => {
            // Give vim time to start
            thread::sleep(Duration::from_millis(300));
            
            println!("Initial vim state:");
            print_snapshot(&pty, "📝");
            
            // Test h (left) - move to beginning of line first
            println!("\n➡️ Pressing '0' (go to line start), then 'l' (right) 5 times...");
            let _ = pty.send_input(b"0");
            thread::sleep(Duration::from_millis(100));
            for _ in 0..5 {
                let _ = pty.send_input(b"l");
                thread::sleep(Duration::from_millis(50));
            }
            print_snapshot(&pty, "➡️");
            
            // Test h (left)
            println!("\n⬅️ Pressing 'h' (left) 3 times...");
            for _ in 0..3 {
                let _ = pty.send_input(b"h");
                thread::sleep(Duration::from_millis(50));
            }
            print_snapshot(&pty, "⬅️");
            
            // Test j (down)
            println!("\n⬇️ Pressing 'j' (down) 5 times...");
            for _ in 0..5 {
                let _ = pty.send_input(b"j");
                thread::sleep(Duration::from_millis(50));
            }
            print_snapshot(&pty, "⬇️");
            
            // Test k (up)
            println!("\n⬆️ Pressing 'k' (up) 3 times...");
            for _ in 0..3 {
                let _ = pty.send_input(b"k");
                thread::sleep(Duration::from_millis(50));
            }
            print_snapshot(&pty, "⬆️");
            
            // Quit vim
            let _ = pty.send_input(b"\x1b:q!\n");
            let _ = child.wait();
        }
        Err(e) => eprintln!("❌ Failed to start vim: {}", e),
    }
}

fn test_vim_scrolling() {
    println!("\n📋 Test 2: Vim scrolling with Ctrl+D and Ctrl+U");
    
    let mut pty = PtyAdapter::new(50, 8);
    
    let mut cmd = Command::new("vim");
    cmd.arg("numbered_lines.txt");
    
    match pty.spawn_command(&mut cmd) {
        Ok(mut child) => {
            thread::sleep(Duration::from_millis(300));
            
            println!("Starting at top of file:");
            print_snapshot(&pty, "🔝");
            
            // Scroll down half page (Ctrl+D)
            println!("\n⬇️ Pressing Ctrl+D (scroll down half page)...");
            let _ = pty.send_input(b"\x04"); // Ctrl+D
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "⬇️");
            
            // Scroll down again
            println!("\n⬇️ Pressing Ctrl+D again...");
            let _ = pty.send_input(b"\x04");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "⬇️");
            
            // Scroll up half page (Ctrl+U)
            println!("\n⬆️ Pressing Ctrl+U (scroll up half page)...");
            let _ = pty.send_input(b"\x15"); // Ctrl+U
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "⬆️");
            
            // Test with 'f' and 'b' for page down/up
            println!("\n📄 Pressing Ctrl+F (page down)...");
            let _ = pty.send_input(b"\x06"); // Ctrl+F
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "📄");
            
            println!("\n📄 Pressing Ctrl+B (page up)...");
            let _ = pty.send_input(b"\x02"); // Ctrl+B
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "📄");
            
            // Quit vim
            let _ = pty.send_input(b"\x1b:q!\n");
            let _ = child.wait();
        }
        Err(e) => eprintln!("❌ Failed to start vim: {}", e),
    }
}

fn test_vim_jump_commands() {
    println!("\n📋 Test 3: Vim jump commands (G, gg, line numbers)");
    
    let mut pty = PtyAdapter::new(55, 12);
    
    let mut cmd = Command::new("vim");
    cmd.arg("numbered_lines.txt");
    
    match pty.spawn_command(&mut cmd) {
        Ok(mut child) => {
            thread::sleep(Duration::from_millis(300));
            
            println!("Starting position:");
            print_snapshot(&pty, "🏁");
            
            // Jump to end of file
            println!("\n🔚 Pressing 'G' (go to end of file)...");
            let _ = pty.send_input(b"G");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "🔚");
            
            // Jump to beginning
            println!("\n🏠 Pressing 'gg' (go to beginning)...");
            let _ = pty.send_input(b"gg");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "🏠");
            
            // Jump to specific line (line 50)
            println!("\n🎯 Going to line 50 (typing '50G')...");
            let _ = pty.send_input(b"50G");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "🎯");
            
            // Jump to line 25
            println!("\n🎯 Going to line 25 (typing ':25' + Enter)...");
            let _ = pty.send_input(b":25\n");
            thread::sleep(Duration::from_millis(150));
            print_snapshot(&pty, "🎯");
            
            // Test word movement
            println!("\n🔤 Testing word movement with 'w' (next word) 3 times...");
            for _ in 0..3 {
                let _ = pty.send_input(b"w");
                thread::sleep(Duration::from_millis(50));
            }
            print_snapshot(&pty, "🔤");
            
            // Test 'b' for back word
            println!("\n🔤 Testing 'b' (previous word) 2 times...");
            for _ in 0..2 {
                let _ = pty.send_input(b"b");
                thread::sleep(Duration::from_millis(50));
            }
            print_snapshot(&pty, "🔤");
            
            // Test end/beginning of line
            println!("\n📍 Testing '$' (end of line)...");
            let _ = pty.send_input(b"$");
            thread::sleep(Duration::from_millis(100));
            print_snapshot(&pty, "📍");
            
            println!("\n📍 Testing '0' (beginning of line)...");
            let _ = pty.send_input(b"0");
            thread::sleep(Duration::from_millis(100));
            print_snapshot(&pty, "📍");
            
            // Quit vim
            let _ = pty.send_input(b"\x1b:q!\n");
            let _ = child.wait();
        }
        Err(e) => eprintln!("❌ Failed to start vim: {}", e),
    }
}

fn print_snapshot(pty: &PtyAdapter, prefix: &str) {
    let snapshot = pty.get_snapshot();
    let lines: Vec<&str> = snapshot.lines().collect();
    
    println!("{} Current vim view ({} lines visible):", prefix, lines.len());
    println!("┌{}┐", "─".repeat(57));
    for line in lines.iter().take(12) {
        let truncated = if line.len() > 55 {
            format!("{}...", &line[..52])
        } else {
            format!("{:<55}", line)
        };
        println!("│{}│", truncated);
    }
    println!("└{}┘", "─".repeat(57));
}