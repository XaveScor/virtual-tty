use std::sync::{Arc, Mutex};

mod ansi;
mod buffer;
mod cursor;
mod errors;

use ansi::{parse_escape_sequence, AnsiCommand, ClearMode};
use buffer::Buffer;
use cursor::Cursor;

pub struct VirtualTty {
    buffer: Arc<Mutex<Buffer>>,
    cursor: Arc<Mutex<Cursor>>,
    width: usize,
    height: usize,
}

impl VirtualTty {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = Buffer::new(width, height);
        let cursor = Cursor::new();

        Self {
            buffer: Arc::new(Mutex::new(buffer)),
            cursor: Arc::new(Mutex::new(cursor)),
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
        self.write_internal(input);
    }

    fn write_internal(&mut self, data: &str) {
        let mut buffer = self.buffer.lock().unwrap();
        let mut cursor = self.cursor.lock().unwrap();

        let mut chars = data.chars();
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Start of escape sequence
                if chars.next() == Some('[') {
                    if let Some(command) = parse_escape_sequence(&mut chars) {
                        self.execute_ansi_command(&command, &mut buffer, &mut cursor);
                    }
                }
            } else if ch == '\r' {
                // Carriage return
                cursor.carriage_return();
            } else if ch == '\n' {
                // Newline
                if cursor.newline(self.height) {
                    buffer.scroll_up();
                }
            } else if ch == '\x08' {
                // Backspace
                cursor.backspace();
            } else {
                // Regular character
                if cursor.row < self.height && cursor.col < self.width {
                    buffer.set_char(cursor.row, cursor.col, ch);
                    if cursor.advance(self.width, self.height) {
                        buffer.scroll_up();
                    }
                }
            }
        }
    }

    fn execute_ansi_command(
        &self,
        command: &AnsiCommand,
        buffer: &mut Buffer,
        cursor: &mut Cursor,
    ) {
        match command {
            AnsiCommand::CursorUp(n) => {
                cursor.move_up(*n);
            }
            AnsiCommand::CursorDown(n) => {
                cursor.move_down(*n, self.height);
            }
            AnsiCommand::CursorForward(n) => {
                cursor.move_forward(*n, self.width);
            }
            AnsiCommand::CursorBack(n) => {
                cursor.move_back(*n);
            }
            AnsiCommand::CursorPosition { row, col } => {
                cursor.set_position(*row, *col, self.height, self.width);
            }
            AnsiCommand::ClearScreen(clear_mode) => match clear_mode {
                ClearMode::Entire => {
                    buffer.clear();
                    cursor.set_position(0, 0, self.height, self.width);
                }
                ClearMode::ToBeginning => {
                    buffer.clear_from_beginning_to_cursor(cursor.row, cursor.col);
                }
                ClearMode::ToEnd => {
                    buffer.clear_from_cursor_to_end(cursor.row, cursor.col);
                }
            },
            AnsiCommand::ClearLine(clear_mode) => match clear_mode {
                ClearMode::Entire => {
                    buffer.clear_entire_line(cursor.row);
                }
                ClearMode::ToBeginning => {
                    buffer.clear_line_from_beginning_to_cursor(cursor.row, cursor.col);
                }
                ClearMode::ToEnd => {
                    buffer.clear_line_from_cursor_to_end(cursor.row, cursor.col);
                }
            },
            AnsiCommand::SetGraphicsRendition => {
                // SGR (Select Graphic Rendition) - ignore for now
            }
        }
    }

    pub fn get_snapshot(&self) -> String {
        let buffer = self.buffer.lock().unwrap();
        buffer.get_snapshot()
    }

    pub fn clear(&mut self) {
        let mut buffer = self.buffer.lock().unwrap();
        let mut cursor = self.cursor.lock().unwrap();

        buffer.clear();
        cursor.set_position(0, 0, self.height, self.width);
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        let cursor = self.cursor.lock().unwrap();
        cursor.get_position()
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
