pub struct Buffer {
    pub lines: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let lines = vec![vec![' '; width]; height];
        Self {
            lines,
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.lines = vec![vec![' '; self.width]; self.height];
    }

    pub fn scroll_up(&mut self) {
        self.lines.remove(0);
        self.lines.push(vec![' '; self.width]);
    }

    pub fn set_char(&mut self, row: usize, col: usize, ch: char) {
        if row < self.height && col < self.width {
            self.lines[row][col] = ch;
        }
    }

    pub fn get_char(&self, row: usize, col: usize) -> Option<char> {
        if row < self.height && col < self.width {
            Some(self.lines[row][col])
        } else {
            None
        }
    }

    pub fn clear_from_cursor_to_end(&mut self, cursor_row: usize, cursor_col: usize) {
        if cursor_row < self.height {
            // Clear rest of current line from cursor position
            for col in cursor_col..self.width {
                self.lines[cursor_row][col] = ' ';
            }
            // Clear all lines below current cursor row
            for row in &mut self.lines[(cursor_row + 1)..self.height] {
                row.fill(' ');
            }
        }
    }

    pub fn clear_from_beginning_to_cursor(&mut self, cursor_row: usize, cursor_col: usize) {
        // Clear all complete lines above current cursor row
        for row in &mut self.lines[0..cursor_row] {
            row.fill(' ');
        }
        // Clear current line from beginning to cursor position (exclusive)
        if cursor_row < self.height {
            for col in 0..cursor_col {
                if col < self.width {
                    self.lines[cursor_row][col] = ' ';
                }
            }
        }
    }

    pub fn clear_line_from_cursor_to_end(&mut self, cursor_row: usize, cursor_col: usize) {
        if cursor_row < self.height {
            for col in cursor_col..self.width {
                self.lines[cursor_row][col] = ' ';
            }
        }
    }

    pub fn clear_line_from_beginning_to_cursor(&mut self, cursor_row: usize, cursor_col: usize) {
        if cursor_row < self.height {
            for col in 0..=cursor_col {
                if col < self.width {
                    self.lines[cursor_row][col] = ' ';
                }
            }
        }
    }

    pub fn clear_entire_line(&mut self, cursor_row: usize) {
        if cursor_row < self.height {
            for col in 0..self.width {
                self.lines[cursor_row][col] = ' ';
            }
        }
    }

    pub fn get_snapshot(&self) -> String {
        let mut result = String::new();
        for (i, row) in self.lines.iter().enumerate() {
            let line: String = row.iter().collect();
            let trimmed = line.trim_end();
            result.push_str(trimmed);
            if i < self.lines.len() - 1 {
                result.push('\n');
            }
        }
        // Remove trailing empty lines
        result.trim_end().to_string()
    }
}
