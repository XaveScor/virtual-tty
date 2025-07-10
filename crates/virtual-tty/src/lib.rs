use std::io::{self, Write};
use std::sync::{Arc, Mutex};

mod ansi;
mod buffer;
mod cursor;
mod errors;
mod state;

use ansi::{parse_escape_sequence, AnsiCommand, AnsiParser, ClearMode, ControlChar, Token};
use state::TtyState;

pub struct VirtualTty {
    state: Arc<Mutex<TtyState>>,
    width: usize,
    height: usize,
}

pub struct VirtualTtyStreams {
    pub stdout: VirtualTtyStdout,
    pub stderr: VirtualTtyStderr,
    tty: VirtualTty,
}

pub struct VirtualTtyStdout {
    state: Arc<Mutex<TtyState>>,
    width: usize,
    height: usize,
}

pub struct VirtualTtyStderr {
    state: Arc<Mutex<TtyState>>,
    width: usize,
    height: usize,
}

impl VirtualTty {
    pub fn new(width: usize, height: usize) -> VirtualTtyStreams {
        let state = TtyState::new(width, height);
        let shared_state = Arc::new(Mutex::new(state));

        let tty = VirtualTty {
            state: shared_state.clone(),
            width,
            height,
        };

        VirtualTtyStreams {
            stdout: VirtualTtyStdout {
                state: shared_state.clone(),
                width,
                height,
            },
            stderr: VirtualTtyStderr {
                state: shared_state.clone(),
                width,
                height,
            },
            tty,
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

    fn process_token(&self, token: Token, state: &mut TtyState) {
        match token {
            Token::Text(text) => {
                for ch in text.chars() {
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
            Token::Command(command) => {
                // Validate command before executing
                if command.validate().is_ok() {
                    self.execute_ansi_command(&command, state);
                }
                // If validation fails, silently ignore the command
            }
            Token::ControlChar(ctrl_char) => {
                match ctrl_char {
                    ControlChar::LineFeed => {
                        if state.cursor.newline(self.height) {
                            state.buffer.scroll_up();
                        }
                    }
                    ControlChar::CarriageReturn => {
                        state.cursor.carriage_return();
                    }
                    ControlChar::Backspace => {
                        state.cursor.backspace();
                    }
                    ControlChar::Tab => {
                        // Simple tab handling - advance to next tab stop (8 chars)
                        let tab_width = 8;
                        let cursor_col = state.cursor.col;
                        let next_tab_stop = ((cursor_col / tab_width) + 1) * tab_width;
                        let spaces_to_add = next_tab_stop - cursor_col;
                        for _ in 0..spaces_to_add {
                            let cursor_row = state.cursor.row;
                            let cursor_col = state.cursor.col;
                            if cursor_row < self.height && cursor_col < self.width {
                                state.buffer.set_char(cursor_row, cursor_col, ' ');
                                if state.cursor.advance(self.width, self.height) {
                                    state.buffer.scroll_up();
                                }
                            }
                        }
                    }
                    ControlChar::Bell => {
                        // Bell character - typically ignored in terminal emulation
                    }
                    ControlChar::VerticalTab => {
                        // Vertical tab - move to next line
                        if state.cursor.newline(self.height) {
                            state.buffer.scroll_up();
                        }
                    }
                    ControlChar::FormFeed => {
                        // Form feed - clear screen and move to top
                        state.buffer.clear();
                        state.cursor.set_position(0, 0, self.height, self.width);
                    }
                }
            }
            Token::Invalid(_) => {
                // Ignore invalid tokens for now
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
}

impl VirtualTtyStreams {
    pub fn get_width(&self) -> usize {
        self.tty.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.tty.get_height()
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.tty.get_size()
    }

    pub fn get_snapshot(&self) -> String {
        self.tty.get_snapshot()
    }

    pub fn clear(&mut self) {
        self.tty.clear()
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        self.tty.get_cursor_position()
    }

    pub fn send_input(&mut self, input: &str) {
        // For input, we'll use the same logic as the original send_input
        let data = input;
        match AnsiParser::parse(data) {
            Ok(tokens) => {
                let mut state = self.tty.state.lock().unwrap();
                for token in tokens {
                    self.tty.process_token(token, &mut state);
                }
            }
            Err(_) => {
                // Fallback to legacy parsing for compatibility
                self.send_input_legacy(input);
            }
        }
    }

    fn send_input_legacy(&mut self, data: &str) {
        let mut state = self.tty.state.lock().unwrap();
        let mut chars = data.chars();
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Start of escape sequence
                if chars.next() == Some('[') {
                    if let Some(command) = parse_escape_sequence(&mut chars) {
                        self.tty.execute_ansi_command(&command, &mut state);
                    }
                }
            } else if ch == '\r' {
                // Carriage return
                state.cursor.carriage_return();
            } else if ch == '\n' {
                // Newline
                if state.cursor.newline(self.tty.height) {
                    state.buffer.scroll_up();
                }
            } else if ch == '\x08' {
                // Backspace
                state.cursor.backspace();
            } else {
                // Regular character
                let cursor_row = state.cursor.row;
                let cursor_col = state.cursor.col;
                if cursor_row < self.tty.height && cursor_col < self.tty.width {
                    state.buffer.set_char(cursor_row, cursor_col, ch);
                    if state.cursor.advance(self.tty.width, self.tty.height) {
                        state.buffer.scroll_up();
                    }
                }
            }
        }
    }
}

impl Write for VirtualTtyStdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let data = String::from_utf8_lossy(buf);
        self.write_internal(&data);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Write for VirtualTtyStderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let data = String::from_utf8_lossy(buf);
        self.write_internal(&data);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl VirtualTtyStdout {
    fn write_internal(&mut self, data: &str) {
        // Use the new tokenized parser
        match AnsiParser::parse(data) {
            Ok(tokens) => {
                let mut state = self.state.lock().unwrap();
                for token in tokens {
                    self.process_token(token, &mut state);
                }
            }
            Err(_) => {
                // Fallback to legacy parsing for compatibility
                self.write_internal_legacy(data);
            }
        }
    }

    fn process_token(&self, token: Token, state: &mut TtyState) {
        match token {
            Token::Text(text) => {
                for ch in text.chars() {
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
            Token::Command(command) => {
                // Validate command before executing
                if command.validate().is_ok() {
                    self.execute_ansi_command(&command, state);
                }
                // If validation fails, silently ignore the command
            }
            Token::ControlChar(ctrl_char) => {
                match ctrl_char {
                    ControlChar::LineFeed => {
                        if state.cursor.newline(self.height) {
                            state.buffer.scroll_up();
                        }
                    }
                    ControlChar::CarriageReturn => {
                        state.cursor.carriage_return();
                    }
                    ControlChar::Backspace => {
                        state.cursor.backspace();
                    }
                    ControlChar::Tab => {
                        // Simple tab handling - advance to next tab stop (8 chars)
                        let tab_width = 8;
                        let cursor_col = state.cursor.col;
                        let next_tab_stop = ((cursor_col / tab_width) + 1) * tab_width;
                        let spaces_to_add = next_tab_stop - cursor_col;
                        for _ in 0..spaces_to_add {
                            let cursor_row = state.cursor.row;
                            let cursor_col = state.cursor.col;
                            if cursor_row < self.height && cursor_col < self.width {
                                state.buffer.set_char(cursor_row, cursor_col, ' ');
                                if state.cursor.advance(self.width, self.height) {
                                    state.buffer.scroll_up();
                                }
                            }
                        }
                    }
                    ControlChar::Bell => {
                        // Bell character - typically ignored in terminal emulation
                    }
                    ControlChar::VerticalTab => {
                        // Vertical tab - move to next line
                        if state.cursor.newline(self.height) {
                            state.buffer.scroll_up();
                        }
                    }
                    ControlChar::FormFeed => {
                        // Form feed - clear screen and move to top
                        state.buffer.clear();
                        state.cursor.set_position(0, 0, self.height, self.width);
                    }
                }
            }
            Token::Invalid(_) => {
                // Ignore invalid tokens for now
            }
        }
    }

    fn write_internal_legacy(&mut self, data: &str) {
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
}

impl VirtualTtyStderr {
    fn write_internal(&mut self, data: &str) {
        // Use the new tokenized parser
        match AnsiParser::parse(data) {
            Ok(tokens) => {
                let mut state = self.state.lock().unwrap();
                for token in tokens {
                    self.process_token(token, &mut state);
                }
            }
            Err(_) => {
                // Fallback to legacy parsing for compatibility
                self.write_internal_legacy(data);
            }
        }
    }

    fn process_token(&self, token: Token, state: &mut TtyState) {
        match token {
            Token::Text(text) => {
                for ch in text.chars() {
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
            Token::Command(command) => {
                // Validate command before executing
                if command.validate().is_ok() {
                    self.execute_ansi_command(&command, state);
                }
                // If validation fails, silently ignore the command
            }
            Token::ControlChar(ctrl_char) => {
                match ctrl_char {
                    ControlChar::LineFeed => {
                        if state.cursor.newline(self.height) {
                            state.buffer.scroll_up();
                        }
                    }
                    ControlChar::CarriageReturn => {
                        state.cursor.carriage_return();
                    }
                    ControlChar::Backspace => {
                        state.cursor.backspace();
                    }
                    ControlChar::Tab => {
                        // Simple tab handling - advance to next tab stop (8 chars)
                        let tab_width = 8;
                        let cursor_col = state.cursor.col;
                        let next_tab_stop = ((cursor_col / tab_width) + 1) * tab_width;
                        let spaces_to_add = next_tab_stop - cursor_col;
                        for _ in 0..spaces_to_add {
                            let cursor_row = state.cursor.row;
                            let cursor_col = state.cursor.col;
                            if cursor_row < self.height && cursor_col < self.width {
                                state.buffer.set_char(cursor_row, cursor_col, ' ');
                                if state.cursor.advance(self.width, self.height) {
                                    state.buffer.scroll_up();
                                }
                            }
                        }
                    }
                    ControlChar::Bell => {
                        // Bell character - typically ignored in terminal emulation
                    }
                    ControlChar::VerticalTab => {
                        // Vertical tab - move to next line
                        if state.cursor.newline(self.height) {
                            state.buffer.scroll_up();
                        }
                    }
                    ControlChar::FormFeed => {
                        // Form feed - clear screen and move to top
                        state.buffer.clear();
                        state.cursor.set_position(0, 0, self.height, self.width);
                    }
                }
            }
            Token::Invalid(_) => {
                // Ignore invalid tokens for now
            }
        }
    }

    fn write_internal_legacy(&mut self, data: &str) {
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
        write!(tty.stdout, "Hello").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Hello     \n
                  \n
                  \n
        ");
    }

    #[test]
    fn test_newline() {
        let mut tty = VirtualTty::new(10, 3);
        write!(tty.stdout, "Line1\nLine2").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Line1     \n
        Line2     \n
                  \n
        ");
    }

    #[test]
    fn test_line_wrap() {
        let mut tty = VirtualTty::new(5, 3);
        write!(tty.stdout, "HelloWorld").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Hello\n
        World\n
             \n
        ");
    }

    #[test]
    fn test_clear_screen() {
        let mut tty = VirtualTty::new(10, 3);
        write!(tty.stdout, "Hello\nWorld").unwrap();
        write!(tty.stdout, "\x1b[2J").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        \n
        \n
        \n
        ");
    }

    #[test]
    fn test_stderr() {
        let mut tty = VirtualTty::new(10, 3);
        write!(tty.stderr, "Error!").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Error!    \n
                  \n
                  \n
        ");
    }

    #[test]
    fn test_scroll() {
        let mut tty = VirtualTty::new(10, 2);
        write!(tty.stdout, "Line1\nLine2\nLine3").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Line2     \n
        Line3     \n
        ");
    }

    #[test]
    fn test_clear() {
        let mut tty = VirtualTty::new(10, 3);
        write!(tty.stdout, "Hello\nWorld").unwrap();
        tty.clear();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        \n
        \n
        \n
        ");
    }

    // =============================================================================
    // STDERR TESTS - Mirror of stdout tests but using stderr_write()
    // =============================================================================

    #[test]
    fn test_stderr_basic_write() {
        let mut tty = VirtualTty::new(10, 3);
        write!(tty.stderr, "Hello").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Hello     \n
                  \n
                  \n
        ");
    }

    #[test]
    fn test_stderr_newline() {
        let mut tty = VirtualTty::new(10, 3);
        write!(tty.stderr, "Line1\nLine2").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Line1     \n
        Line2     \n
                  \n
        ");
    }

    #[test]
    fn test_stderr_line_wrap() {
        let mut tty = VirtualTty::new(5, 3);
        write!(tty.stderr, "HelloWorld").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Hello\n
        World\n
             \n
        ");
    }

    #[test]
    fn test_stderr_clear_screen() {
        let mut tty = VirtualTty::new(10, 3);
        write!(tty.stderr, "Hello\nWorld").unwrap();
        write!(tty.stderr, "\x1b[2J").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        \n
        \n
        \n
        ");
    }

    #[test]
    fn test_stderr_scroll() {
        let mut tty = VirtualTty::new(10, 2);
        write!(tty.stderr, "Line1\nLine2\nLine3").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Line2     \n
        Line3     \n
        ");
    }

    #[test]
    fn test_mixed_stdout_stderr() {
        let mut tty = VirtualTty::new(15, 3);
        write!(tty.stdout, "Hello").unwrap();
        write!(tty.stderr, " World").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        Hello World    \n
                       \n
                       \n
        ");
    }

    #[test]
    fn test_stderr_with_ansi_escape() {
        let mut tty = VirtualTty::new(10, 3);
        write!(tty.stderr, "Hello").unwrap();
        write!(tty.stderr, "\x1b[1A").unwrap(); // Move up 1 line
        write!(tty.stderr, "X").unwrap();
        let snapshot = tty.get_snapshot();
        insta::assert_snapshot!(snapshot, @r"
        HelloX    \n
                  \n
                  \n
        ");
    }
}
