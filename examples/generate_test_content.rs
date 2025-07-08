use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Generating test content files for scrolling examples...\n");

    // Generate a file with numbered lines
    generate_numbered_lines()?;
    
    // Generate a file with long lines that will wrap
    generate_long_lines()?;
    
    // Generate a large file with diverse content
    generate_large_content()?;
    
    println!("✅ All test files generated successfully!");
    println!("\nGenerated files:");
    println!("- numbered_lines.txt (100 lines with numbers)");
    println!("- long_lines.txt (lines that exceed terminal width)"); 
    println!("- large_content.txt (diverse content for testing)");
    
    Ok(())
}

fn generate_numbered_lines() -> io::Result<()> {
    let mut file = File::create("numbered_lines.txt")?;
    
    for i in 1..=100 {
        writeln!(file, "Line {:3}: This is line number {} with some additional content to make it interesting", i, i)?;
    }
    
    println!("📝 Generated numbered_lines.txt (100 lines)");
    Ok(())
}

fn generate_long_lines() -> io::Result<()> {
    let mut file = File::create("long_lines.txt")?;
    
    for i in 1..=30 {
        writeln!(file, "Line {}: This is a very long line that will definitely exceed the typical terminal width and should wrap around multiple times when displayed in a small terminal window. This tests horizontal scrolling and line wrapping behavior in various terminal emulators and pagers like less.", i)?;
    }
    
    println!("📝 Generated long_lines.txt (30 long lines)");
    Ok(())
}

fn generate_large_content() -> io::Result<()> {
    let mut file = File::create("large_content.txt")?;
    
    // Header
    writeln!(file, "LARGE CONTENT FILE FOR TERMINAL TESTING")?;
    writeln!(file, "========================================")?;
    writeln!(file)?;
    
    // Different types of content
    writeln!(file, "SECTION 1: SHORT LINES")?;
    writeln!(file, "----------------------")?;
    for i in 1..=15 {
        writeln!(file, "Short line {}", i)?;
    }
    writeln!(file)?;
    
    writeln!(file, "SECTION 2: MEDIUM LINES")?;
    writeln!(file, "-----------------------")?;
    for i in 1..=15 {
        writeln!(file, "Medium length line {} with some additional content", i)?;
    }
    writeln!(file)?;
    
    writeln!(file, "SECTION 3: LONG LINES")?;
    writeln!(file, "---------------------")?;
    for i in 1..=15 {
        writeln!(file, "Long line {} that contains quite a bit more text and should definitely wrap around in smaller terminal windows, testing the line wrapping and horizontal scrolling capabilities", i)?;
    }
    writeln!(file)?;
    
    writeln!(file, "SECTION 4: MIXED CONTENT")?;
    writeln!(file, "------------------------")?;
    writeln!(file, "• Bullet point 1")?;
    writeln!(file, "• Bullet point 2 with more text")?;
    writeln!(file, "• Bullet point 3 with even more text that might wrap")?;
    writeln!(file)?;
    writeln!(file, "Code block example:")?;
    writeln!(file, "fn main() {{")?;
    writeln!(file, "    println!(\"Hello, world!\");")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    
    writeln!(file, "SECTION 5: NUMBERED CONTENT")?;
    writeln!(file, "---------------------------")?;
    for i in 1..=25 {
        writeln!(file, "{:2}. Item number {} in the list", i, i)?;
    }
    writeln!(file)?;
    
    writeln!(file, "END OF FILE")?;
    writeln!(file, "Total lines: approximately 100+")?;
    
    println!("📝 Generated large_content.txt (diverse content)");
    Ok(())
}