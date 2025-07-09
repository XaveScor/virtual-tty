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

    pub fn resize_from(old_buffer: &Buffer, new_width: usize, new_height: usize) -> Self {
        let mut new_lines = vec![vec![' '; new_width]; new_height];

        // Copy existing content within the bounds of the new buffer
        let copy_height = old_buffer.height.min(new_height);
        let copy_width = old_buffer.width.min(new_width);

        for row in 0..copy_height {
            for col in 0..copy_width {
                new_lines[row][col] = old_buffer.lines[row][col];
            }
        }

        Self {
            lines: new_lines,
            width: new_width,
            height: new_height,
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
        result.push('\n');
        for row in &self.lines {
            let line: String = row.iter().collect();
            result.push_str(&line);
            // Add \\n for visual clarity in tests to show line endings, then actual \n for line break
            // Example output:
            // "
            // Hello     \n
            // World     \n
            //           \n
            // "
            result.push_str("\\n\n");
        }
        result
    }
}
