use std::sync::{Arc, Mutex};

mod ansi;
mod buffer;
mod cursor;
mod errors;
mod state;

use ansi::{parse_escape_sequence, AnsiCommand, ClearMode};
use state::TtyState;

pub struct VirtualTty {
    state: Arc<Mutex<TtyState>>,
    width: usize,
    height: usize,
}

impl VirtualTty {
    pub fn new(width: usize, height: usize) -> Self {
        let state = TtyState::new(width, height);

        Self {
            state: Arc::new(Mutex::new(state)),
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
        let mut state = self.state.lock().unwrap();

        let mut chars = data.chars();
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Start of escape sequence
                if chars.next() == Some('[') {
                    if let Some(command) = parse_escape_sequence(&mut chars) {
                        self.execute_ansi_command(&command, &mut state);
                    }
                }
            } else if ch == '\r' {
                // Carriage return
                state.cursor.carriage_return();
            } else if ch == '\n' {
                // Newline
                if state.cursor.newline(self.height) {
                    state.buffer.scroll_up();
                }
            } else if ch == '\x08' {
                // Backspace
                state.cursor.backspace();
            } else {
                // Regular character
                let cursor_row = state.cursor.row;
                let cursor_col = state.cursor.col;
                if cursor_row < self.height && cursor_col < self.width {
                    state.buffer.set_char(cursor_row, cursor_col, ch);
                    if state.cursor.advance(self.width, self.height) {
                        state.buffer.scroll_up();
                    }
                }
            }
        }
    }

    fn execute_ansi_command(&self, command: &AnsiCommand, state: &mut TtyState) {
        match command {
            AnsiCommand::CursorUp(n) => {
                state.cursor.move_up(*n);
            }
            AnsiCommand::CursorDown(n) => {
                state.cursor.move_down(*n, self.height);
            }
            AnsiCommand::CursorForward(n) => {
                state.cursor.move_forward(*n, self.width);
            }
            AnsiCommand::CursorBack(n) => {
                state.cursor.move_back(*n);
            }
            AnsiCommand::CursorPosition { row, col } => {
                state
                    .cursor
                    .set_position(*row, *col, self.height, self.width);
            }
            AnsiCommand::ClearScreen(clear_mode) => match clear_mode {
                ClearMode::Entire => {
                    state.buffer.clear();
                    state.cursor.set_position(0, 0, self.height, self.width);
                }
                ClearMode::ToBeginning => {
                    let cursor_row = state.cursor.row;
                    let cursor_col = state.cursor.col;
                    state
                        .buffer
                        .clear_from_beginning_to_cursor(cursor_row, cursor_col);
                }
                ClearMode::ToEnd => {
                    let cursor_row = state.cursor.row;
                    let cursor_col = state.cursor.col;
                    state
                        .buffer
                        .clear_from_cursor_to_end(cursor_row, cursor_col);
                }
            },
            AnsiCommand::ClearLine(clear_mode) => match clear_mode {
                ClearMode::Entire => {
                    let cursor_row = state.cursor.row;
                    state.buffer.clear_entire_line(cursor_row);
                }
                ClearMode::ToBeginning => {
                    let cursor_row = state.cursor.row;
                    let cursor_col = state.cursor.col;
                    state
                        .buffer
                        .clear_line_from_beginning_to_cursor(cursor_row, cursor_col);
                }
                ClearMode::ToEnd => {
                    let cursor_row = state.cursor.row;
                    let cursor_col = state.cursor.col;
                    state
                        .buffer
                        .clear_line_from_cursor_to_end(cursor_row, cursor_col);
                }
            },
            AnsiCommand::SetGraphicsRendition => {
                // SGR (Select Graphic Rendition) - ignore for now
            }
        }
    }

    pub fn get_snapshot(&self) -> String {
        let state = self.state.lock().unwrap();
        state.get_snapshot()
    }

    pub fn clear(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.clear(self.width, self.height);
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        let state = self.state.lock().unwrap();
        state.get_cursor_position()
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
