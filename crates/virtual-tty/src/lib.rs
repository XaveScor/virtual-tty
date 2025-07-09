use std::sync::{Arc, Mutex};

pub struct VirtualTty {
    buffer: Arc<Mutex<Vec<Vec<char>>>>,
    cursor_row: Arc<Mutex<usize>>,
    cursor_col: Arc<Mutex<usize>>,
    width: usize,
    height: usize,
}

impl VirtualTty {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![vec![' '; width]; height];
        Self {
            buffer: Arc::new(Mutex::new(buffer)),
            cursor_row: Arc::new(Mutex::new(0)),
            cursor_col: Arc::new(Mutex::new(0)),
            width,
            height,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn stdout_write(&mut self, data: &str) {
        self.write_internal(data);
    }

    pub fn stderr_write(&mut self, data: &str) {
        self.write_internal(data);
    }

    pub fn send_input(&mut self, input: &str) {
        // For testing purposes, this can trigger input handlers
        // Implementation depends on the application being tested
        self.write_internal(input);
    }

    fn write_internal(&mut self, data: &str) {
        let buffer = self.buffer.clone();
        let cursor_row = self.cursor_row.clone();
        let cursor_col = self.cursor_col.clone();
        Self::process_output(
            data,
            &buffer,
            &cursor_row,
            &cursor_col,
            self.width,
            self.height,
        );
    }

    fn process_output(
        data: &str,
        buffer: &Arc<Mutex<Vec<Vec<char>>>>,
        cursor_row: &Arc<Mutex<usize>>,
        cursor_col: &Arc<Mutex<usize>>,
        width: usize,
        height: usize,
    ) {
        let mut buffer = buffer.lock().unwrap();
        let mut row = cursor_row.lock().unwrap();
        let mut col = cursor_col.lock().unwrap();

        let mut chars = data.chars();
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Start of escape sequence
                if chars.next() == Some('[') {
                    Self::handle_escape_sequence_static(
                        &mut chars,
                        &mut buffer,
                        &mut row,
                        &mut col,
                        width,
                        height,
                    );
                }
            } else if ch == '\r' {
                // Carriage return
                *col = 0;
            } else if ch == '\n' {
                // Newline
                *col = 0;
                *row += 1;
                if *row >= height {
                    // Scroll up
                    buffer.remove(0);
                    buffer.push(vec![' '; width]);
                    *row = height - 1;
                }
            } else if ch == '\x08' {
                // Backspace
                if *col > 0 {
                    *col -= 1;
                }
            } else {
                // Regular character
                if *col < width && *row < height {
                    buffer[*row][*col] = ch;
                    *col += 1;

                    // Wrap to next line if needed
                    if *col >= width {
                        *col = 0;
                        *row += 1;
                        if *row >= height {
                            // Scroll up
                            buffer.remove(0);
                            buffer.push(vec![' '; width]);
                            *row = height - 1;
                        }
                    }
                }
            }
        }
    }

    fn handle_escape_sequence_static(
        chars: &mut std::str::Chars,
        buffer: &mut Vec<Vec<char>>,
        cursor_row: &mut usize,
        cursor_col: &mut usize,
        width: usize,
        height: usize,
    ) {
        let mut params = String::new();
        let mut cmd = ' ';

        // Read until we find the command character
        for ch in chars {
            if ch.is_ascii_alphabetic() || ch == '~' {
                cmd = ch;
                break;
            }
            params.push(ch);
        }

        match cmd {
            'A' => {
                // Cursor up
                let n = params.parse::<usize>().unwrap_or(1);
                *cursor_row = cursor_row.saturating_sub(n);
            }
            'B' => {
                // Cursor down
                let n = params.parse::<usize>().unwrap_or(1);
                *cursor_row = (*cursor_row + n).min(height - 1);
            }
            'C' => {
                // Cursor forward
                let n = params.parse::<usize>().unwrap_or(1);
                *cursor_col = (*cursor_col + n).min(width - 1);
            }
            'D' => {
                // Cursor back
                let n = params.parse::<usize>().unwrap_or(1);
                *cursor_col = cursor_col.saturating_sub(n);
            }
            'H' | 'f' => {
                // Cursor position
                let parts: Vec<&str> = params.split(';').collect();
                let row = parts
                    .first()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1)
                    .saturating_sub(1);
                let col = parts
                    .get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1)
                    .saturating_sub(1);
                *cursor_row = row.min(height - 1);
                *cursor_col = col.min(width - 1);
            }
            'J' => {
                // Clear screen
                if params == "2" {
                    // Clear entire screen and move cursor to home
                    *buffer = vec![vec![' '; width]; height];
                    *cursor_row = 0;
                    *cursor_col = 0;
                } else if params == "0" || params.is_empty() {
                    // Clear from cursor to end of screen
                    // Clear rest of current line from cursor position
                    for col in *cursor_col..width {
                        buffer[*cursor_row][col] = ' ';
                    }
                    // Clear all lines below current cursor row
                    for row in &mut buffer[(*cursor_row + 1)..height] {
                        row.fill(' ');
                    }
                    // Cursor position remains unchanged
                }
            }
            'K' => {
                // Clear line operations
                if params == "1" {
                    // Clear from beginning of line to cursor position
                    for col in 0..=*cursor_col {
                        if col < width {
                            buffer[*cursor_row][col] = ' ';
                        }
                    }
                } else if params == "2" {
                    // Clear entire line
                    for col in 0..width {
                        buffer[*cursor_row][col] = ' ';
                    }
                } else {
                    // Clear to end of line (default behavior)
                    for col in *cursor_col..width {
                        buffer[*cursor_row][col] = ' ';
                    }
                }
            }
            'm' => {
                // SGR (Select Graphic Rendition) - ignore for now
            }
            _ => {
                // Unknown command, ignore
            }
        }
    }

    pub fn get_snapshot(&self) -> String {
        let buffer = self.buffer.lock().unwrap();
        let mut result = String::new();
        for (i, row) in buffer.iter().enumerate() {
            let line: String = row.iter().collect();
            let trimmed = line.trim_end();
            result.push_str(trimmed);
            if i < buffer.len() - 1 {
                result.push('\n');
            }
        }
        // Remove trailing empty lines
        result.trim_end().to_string()
    }

    pub fn clear(&mut self) {
        let mut buffer = self.buffer.lock().unwrap();
        let mut row = self.cursor_row.lock().unwrap();
        let mut col = self.cursor_col.lock().unwrap();

        *buffer = vec![vec![' '; self.width]; self.height];
        *row = 0;
        *col = 0;
    }

    /// Get current cursor position (row, col)
    pub fn get_cursor_position(&self) -> (usize, usize) {
        let row = self.cursor_row.lock().unwrap();
        let col = self.cursor_col.lock().unwrap();
        (*row, *col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tty = VirtualTty::new(80, 24);
        assert_eq!(tty.get_width(), 80);
        assert_eq!(tty.get_height(), 24);
        assert_eq!(tty.get_size(), (80, 24));
    }

    #[test]
    fn test_basic_write() {
        let mut tty = VirtualTty::new(10, 3);
        tty.stdout_write("Hello");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Hello");
    }

    #[test]
    fn test_newline() {
        let mut tty = VirtualTty::new(10, 3);
        tty.stdout_write("Line1\nLine2");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Line1\nLine2");
    }

    #[test]
    fn test_line_wrap() {
        let mut tty = VirtualTty::new(5, 3);
        tty.stdout_write("HelloWorld");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Hello\nWorld");
    }

    #[test]
    fn test_clear_screen() {
        let mut tty = VirtualTty::new(10, 3);
        tty.stdout_write("Hello\nWorld");
        tty.stdout_write("\x1b[2J");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "");
    }

    #[test]
    fn test_stderr() {
        let mut tty = VirtualTty::new(10, 3);
        tty.stderr_write("Error!");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Error!");
    }

    #[test]
    fn test_scroll() {
        let mut tty = VirtualTty::new(10, 2);
        tty.stdout_write("Line1\nLine2\nLine3");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Line2\nLine3");
    }

    #[test]
    fn test_clear() {
        let mut tty = VirtualTty::new(10, 3);
        tty.stdout_write("Hello\nWorld");
        tty.clear();
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "");
    }

    // =============================================================================
    // STDERR TESTS - Mirror of stdout tests but using stderr_write()
    // =============================================================================

    #[test]
    fn test_stderr_basic_write() {
        let mut tty = VirtualTty::new(10, 3);
        tty.stderr_write("Hello");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Hello");
    }

    #[test]
    fn test_stderr_newline() {
        let mut tty = VirtualTty::new(10, 3);
        tty.stderr_write("Line1\nLine2");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Line1\nLine2");
    }

    #[test]
    fn test_stderr_line_wrap() {
        let mut tty = VirtualTty::new(5, 3);
        tty.stderr_write("HelloWorld");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Hello\nWorld");
    }

    #[test]
    fn test_stderr_clear_screen() {
        let mut tty = VirtualTty::new(10, 3);
        tty.stderr_write("Hello\nWorld");
        tty.stderr_write("\x1b[2J");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "");
    }

    #[test]
    fn test_stderr_scroll() {
        let mut tty = VirtualTty::new(10, 2);
        tty.stderr_write("Line1\nLine2\nLine3");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Line2\nLine3");
    }

    #[test]
    fn test_mixed_stdout_stderr() {
        let mut tty = VirtualTty::new(15, 3);
        tty.stdout_write("Hello");
        tty.stderr_write(" World");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "Hello World");
    }

    #[test]
    fn test_stderr_with_ansi_escape() {
        let mut tty = VirtualTty::new(10, 3);
        tty.stderr_write("Hello");
        tty.stderr_write("\x1b[1A"); // Move up 1 line
        tty.stderr_write("X");
        let snapshot = tty.get_snapshot();
        assert_eq!(snapshot, "HelloX");
    }
}
